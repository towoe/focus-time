use crate::cli::Cli;
use std::time::Duration;

use log::{debug, error, info};

/// Configuration for the focus timer.
pub struct Config {
    /// Duration of the focus timer.
    pub duration: Duration,
    pub no_notification: bool,
    pub keep_status_bar: bool,
}

impl Config {
    /// Creates a new `Config` instance from the given command line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A reference to the `Cli` struct containing the command line arguments.
    ///
    /// # Returns
    ///
    /// A new `Config` instance with the specified duration.
    pub fn set(args: &Cli) -> Result<Self, String> {
        let duration = match parse_duration(&args.duration) {
            Some(d) => d,
            // If no value can be extracted, propagate this to the caller
            None => return Err("Invalid duration specified".to_string()),
        };
        info!("Duration set to: {:?}", duration);

        let no_notification = args.no_notification;
        let keep_status_bar = args.keep_status_bar;

        debug!("Notifications requested: {}", !no_notification);
        debug!("Status bar hide requested: {}", !keep_status_bar);

        Ok(Self {
            duration,
            no_notification,
            keep_status_bar,
        })
    }
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
            error!("Invalid duration unit: {}", unit_part);
            None
        }
    }
}
