use crate::cli::Cli;
use serde::Deserialize;
use std::{path::Path, time::Duration};

use log::{debug, error, trace};

/// Configuration for the focus timer.
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct FileConfig {
    duration: Option<String>,
    no_notification: Option<bool>,
    keep_status_bar: Option<bool>,
    print_time: Option<bool>,
}

#[derive(Debug)]
pub struct Config {
    /// Duration of the focus timer.
    pub duration: Duration,
    pub no_notification: bool,
    pub keep_status_bar: bool,
    pub print_time: bool,
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
    pub fn new(args: &Cli) -> Result<Self, String> {
        let config = load_from_file(args.config.clone()).unwrap_or_default();

        // If a duration is specifed in the arguments, use this, otherwise check the value from the
        // config file

        let no_notification = args.no_notification || config.no_notification.unwrap_or(false);
        let keep_status_bar = args.keep_status_bar || config.keep_status_bar.unwrap_or(false);
        let print_time = args.print_time || config.print_time.unwrap_or(false);
        let duration = get_duration(&args.duration, &config.duration)?;

        let config = Self {
            duration,
            no_notification,
            keep_status_bar,
            print_time,
        };
        trace!("Using configuration: {:?}", config);
        Ok(config)
    }
}

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

fn load_from_file(argument_path: Option<String>) -> Option<FileConfig> {
    // Check if the user specified a config file
    let config_path = if let Some(path) = argument_path {
        Path::new(&path).to_path_buf()
    } else {
        dirs::config_dir()
            .expect("Failed to get configuration directory")
            .join("focus-time")
            .join("config.toml")
    };
    debug!("Reading configuration from: {:?}", config_path);
    // Make sure the file exists
    if !Path::new(&config_path).exists() {
        return None;
    }
    debug!("Reading content from: {:?}", config_path);
    let config_content =
        std::fs::read_to_string(&config_path).expect("Failed to read configuration file");
    trace!("Parsing: {:?}", config_content);

    match toml::from_str(&config_content) {
        Ok(config) => {
            trace!("Parsed config: {:?}", config);
            return Some(config);
        }
        Err(e) => {
            debug!("Config file could not be loaded: {}", e);
        }
    }
    None
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
