use crate::cli::Cli;
use crate::config::ConfigFile;
use crate::notification_interface::NotificationInterface;
use crate::sway_ipc_interface::SwayIpcInterface;
use crate::swaync_interface::SwayNCInterface;
use crate::timer::Timer;

use anyhow::Result;

use crate::focus_interface::FocusTime;
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::zvariant::Value;
use zbus::Connection;

use log::{debug, error, trace};

#[derive(PartialEq)]
pub enum AbortSignal {
    CtrlC,
    Dbus,
}

/// Configuration for the focus timer.
/// This is the config derived from the config file and the command line arguments. It's used to
/// control the behaviour of the focus timer.
#[derive(Debug)]
pub struct FocusConfig {
    duration: Duration,
    no_notification: bool,
    keep_status_bar: bool,
    print_time: bool,
}

/// Creates a `FocusConfig` from the provided `ConfigFile` and `Cli` arguments.
///
/// # Arguments
///
/// * `file_config` - A `ConfigFile` containing configuration loaded from a file.
/// * `args` - A `Cli` struct containing command line arguments.
///
/// # Returns
///
/// A `FocusConfig` struct containing the merged configuration.
pub fn create_config(file_config: ConfigFile, args: Cli) -> FocusConfig {
    FocusConfig {
        duration: get_duration(&args.duration, &file_config.duration).unwrap(),
        no_notification: args.no_notification || file_config.no_notification.unwrap_or(false),
        keep_status_bar: args.keep_status_bar || file_config.keep_status_bar.unwrap_or(false),
        print_time: args.print_time || file_config.print_time.unwrap_or(false),
    }
}

/// Helper function to extract the value for the duration from multiple sources. If a value is
/// specified, but not in the correct way, and error is returned without checking other values.
/// This behaviour is inteded to prevent undesired default time durations when the supplied value
/// was incorrect.
fn get_duration(
    from_arg: &Option<String>,
    from_config: &Option<String>,
) -> Result<Duration, String> {
    trace!(
        "Parsing duration: argument: {:?} - config:{:?}",
        from_arg,
        from_config
    );
    if let Some(duration) = from_arg {
        if let Some(duration) = parse_duration(duration) {
            debug!("Using duration from argument: {:?}", duration);
            return Ok(duration);
        } else {
            return Err(format!("Invalid duration: '{}'", duration));
        }
    } else if let Some(duration) = from_config {
        if let Some(duration) = parse_duration(duration) {
            debug!("Using duration from config: {:?}", duration);
            return Ok(duration);
        } else {
            return Err(format!("Invalid duration: '{}'", duration));
        }
    }
    debug!("Using default duration: 25 minutes");
    Ok(Duration::from_secs(25 * 60))
}

/// Parses a duration string and returns a `Duration` object if the input is valid.
///
/// # Arguments
///
/// * `input` - A string slice that holds the duration to be parsed.
///
/// # Returns
///
/// * `Option<Duration>` - Returns `Some(Duration)` if the input is valid, otherwise `None`.
pub fn parse_duration(input: &str) -> Option<Duration> {
    if input.is_empty() {
        return None;
    }

    let (number_part, unit_part) = input
        .trim()
        .chars()
        .partition::<String, _>(|c| c.is_ascii_digit());

    let value: u64 = match number_part.parse() {
        Ok(num) => num,
        Err(_) => return None,
    };

    match unit_part.as_str() {
        "s" => Some(Duration::from_secs(value)),
        "m" => Some(Duration::from_secs(value * 60)),
        "h" => Some(Duration::from_secs(value * 60 * 60)),
        "d" => Some(Duration::from_secs(value * 60 * 60 * 24)),
        _ => {
            error!("Invalid duration unit: '{}'", unit_part);
            None
        }
    }
}

pub struct Focus {
    /// Configuration
    config: FocusConfig,
    timer: Timer,
    rx: Mutex<Option<oneshot::Receiver<AbortSignal>>>,
    tx: Arc<Mutex<Option<oneshot::Sender<AbortSignal>>>>,
}

pub fn new(config: FocusConfig) -> Focus {
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
