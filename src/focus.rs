use crate::config::Config;
use crate::display;
use crate::notification_interface::NotificationInterface;
use crate::sway_ipc_interface::SwayIpcInterface;
use crate::swaync_interface::SwayNCInterface;

use anyhow::Result;

use crate::focus_interface::FocusTime;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::zvariant::Value;
use zbus::Connection;

use log::debug;

pub async fn run(config: Config) -> Result<()> {
    // Initialize the interfaces
    let swaync = SwayNCInterface::new().await?;
    let mut sway = SwayIpcInterface::new().await?;

    // Open channels to communicate with the Ctrl+C handler
    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // Set the tools to the desired state
    swaync.enable_dnd().await?;
    if !config.keep_status_bar {
        sway.set_bar_mode_invisible().await?;
    }

    // Set the Ctrl+C handler
    // This is needed so the program does not just end on Ctrl+c, but clean up
    // before. For example, unset the do not disturb state.
    let tx_clone = Arc::clone(&tx);
    ctrlc::set_handler(move || {
        let mut tx_lock = tx_clone.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(());
        }
    })
    .expect("Error setting Ctrl+C handler");

    if config.print_time {
        tokio::spawn(display::print_remaining_time(config.duration));
    }

    let _dbus_conn = start_dbus_service(config.duration, Arc::clone(&tx)).await?;

    let mut timed_end = true;

    // Wait for the `duration` specified time or a Ctrl+C signal
    tokio::select! {
        _ = sleep(config.duration) => {},
        _ = rx => {
            timed_end = false;
            debug!("\nReceived Ctrl+C, starting cleanup...");
        },
    }
    // Make sure the cursor is shown. Should not be a problem if it was not disabled.
    print!("\x1B[?25h"); // Show cursor

    // Restore the tools and notify the user
    swaync.disable_dnd().await?;

    if !config.keep_status_bar {
        sway.set_bar_mode_dock().await?;
    }

    let mut hints = HashMap::new();
    hints.insert("urgency", &Value::U8(2));

    if !config.no_notification && timed_end {
        let notify = NotificationInterface::new().await?;
        let _ = notify
            .notify(
                "Focus time over",
                &format!("{:?} have passed", config.duration),
                hints,
            )
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
async fn start_dbus_service(
    duration: Duration,
    tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
) -> Result<Connection> {
    debug!("Starting D-Bus service");
    let conn = Connection::session().await?;
    conn.object_server()
        .at(
            "/org/towoe/FocusTime",
            FocusTime {
                duration,
                start: std::time::Instant::now(),
                tx,
            },
        )
        .await?;
    conn.request_name("org.towoe.FocusTime").await?;
    debug!("mhm");
    Ok(conn)
}
