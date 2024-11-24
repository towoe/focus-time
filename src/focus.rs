use crate::notification_interface::NotificationInterface;
use crate::sway_ipc_interface::SwayIpcInterface;
use crate::swaync_interface::SwayNCInterface;
use crate::{config::Config, timer::Timer};

use anyhow::Result;

use crate::focus_interface::FocusTime;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::zvariant::Value;
use zbus::Connection;

use log::debug;

#[derive(PartialEq)]
pub enum AbortSignal {
    CtrlC,
    Dbus,
}

pub struct Focus {
    /// Configuration
    config: Config,
    timer: Timer,
    rx: Mutex<Option<oneshot::Receiver<AbortSignal>>>,
    tx: Arc<Mutex<Option<oneshot::Sender<AbortSignal>>>>,
}

pub fn new(config: Config) -> Focus {
    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));
    let rx = Mutex::new(Some(rx));
    let timer = Timer::new(config.duration);
    Focus {
        config,
        timer,
        rx,
        tx,
    }
}

impl Focus {
    pub async fn run(&self) -> Result<()> {
        // Initialize the interfaces
        let swaync = SwayNCInterface::new().await?;
        let mut sway = SwayIpcInterface::new().await?;

        // Set the tools to the desired state
        swaync.enable_dnd().await?;
        if !self.config.keep_status_bar {
            sway.set_bar_mode_invisible().await?;
        }

        // Set the Ctrl+C handler
        // This is needed so the program does not just end on Ctrl+c, but clean up
        // before. For example, unset the do not disturb state.
        let tx_clone = Arc::clone(&self.tx);
        ctrlc::set_handler(move || {
            let mut tx_lock = tx_clone.lock().unwrap();
            if let Some(tx) = tx_lock.take() {
                let _ = tx.send(AbortSignal::CtrlC);
            }
        })
        .expect("Error setting Ctrl+C handler");

        if self.config.print_time {
            tokio::spawn(crate::timer::print_remaining_time(self.timer));
        }

        let _dbus_conn = self.start_dbus_service().await?;

        let mut timer_aborted: Option<AbortSignal> = None;

        let rx = self.rx.lock().unwrap().take().unwrap();
        // Wait for the `duration` specified time or a Ctrl+C signal
        tokio::select! {
                _ = sleep(self.config.duration) => {},
                signal = rx => {
                    match signal {
                    Ok(AbortSignal::CtrlC) => {
                        timer_aborted = Some(AbortSignal::CtrlC);
                        println!("\x1B[2K\rFocus timer aborted at: {}", self.timer);
                        debug!("\nReceived Ctrl+C, starting cleanup...");
                    },
                    Ok(AbortSignal::Dbus) => {
                        timer_aborted = Some(AbortSignal::Dbus);
                        debug!("\nReceived D-Bus signal, starting cleanup...");
                    },
                    Err(_) => {
                        debug!("\nReceived error from channel, starting cleanup...");
                    },
                }
            },
        }
        // Make sure the cursor is shown. Should not be a problem if it was not disabled.
        print!("\x1B[?25h"); // Show cursor

        // Restore the tools and notify the user
        swaync.disable_dnd().await?;

        if !self.config.keep_status_bar {
            sway.set_bar_mode_dock().await?;
        }

        let mut hints = HashMap::new();
        hints.insert("urgency", &Value::U8(2));

        if timer_aborted == Some(AbortSignal::Dbus)
            || (!self.config.no_notification && timer_aborted.is_none())
        {
            let notify = NotificationInterface::new().await?;
            let _ = notify
                .notify("Focus time over", &format!("{}", self.timer), hints)
                .await?;
        }

        Ok(())
    }

    /// Starts a D-Bus service that provides a FocusTime interface.
    /// This interface can be used to query the remaining time of the focus timer.
    /// The service is registered under the name `org.towoe.FocusTime`.
    /// The interface provides a method `get_remaining_time` that returns the remaining time in HH:MM:SS format.
    ///
    /// # Arguments
    ///
    /// * `duration` - The total duration of the focus timer
    ///
    /// # Returns
    ///
    /// A D-Bus connection to the service
    ///
    /// # Client example
    ///
    /// $ busctl --user call org.towoe.FocusTime /org/towoe/FocusTime org.towoe.FocusTime GetRemainingTime
    async fn start_dbus_service(&self) -> Result<Connection> {
        debug!("Starting D-Bus service");
        let conn = Connection::session().await?;
        let tx = Arc::clone(&self.tx);
        conn.object_server()
            .at(
                "/org/towoe/FocusTime",
                FocusTime {
                    timer: self.timer,
                    tx,
                },
            )
            .await?;
        conn.request_name("org.towoe.FocusTime").await?;
        Ok(conn)
    }
}
