/// This module handles the configuration from the config file.
/// The config is automatically loaded when the program starts. The default file location is
/// `XDG_CONFIG_HOME/focus-time/config.toml`. Another file can be specified as an argument when
/// running the program.
use serde::Deserialize;
use std::path::Path;

use log::{debug, trace};

/// Configuration for the focus timer.
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct ConfigFile {
    /// Duration of the focus timer.
    pub duration: Option<String>,
    /// Whether to disable notifications.
    pub no_notification: Option<bool>,
    /// Whether to keep the status bar visible.
    pub keep_status_bar: Option<bool>,
    /// Whether to print the remaining time.
    pub print_time: Option<bool>,
}

/// Loads the configuration from a file.
///
/// # Arguments
///
/// * `cli_config` - An optional string containing the path to the configuration file specified by
///   the user.
///
/// # Returns
///
/// A `Result` containing the `ConfigFile` if successful, or a `String` error message if the file
/// could not be loaded or parsed.
pub fn load_from_file(cli_config: &Option<String>) -> Result<ConfigFile, String> {
    // Check if the user specified a different location for the config file
    // otherwise use the default location
    let config_path = if let Some(path) = cli_config {
        let config_path = Path::new(&path).to_path_buf();
        debug!(
            "Trying to access config file from command argument `{:?}`",
            config_path
        );
        // If the user specified a path, but the file does not exist, return with an error now as
        // the following steps would not be what the user might expect
        if !config_path.exists() {
            return Err(format!("Configuration file not found: {config_path:?}"));
        }
        config_path
    } else {
        dirs::config_dir()
            .expect("Failed to get configuration directory")
            .join("focus-time")
            .join("config.toml")
    };
    debug!("Using config file `{:?}`", config_path);

    if config_path.exists() {
        let config_content =
            std::fs::read_to_string(&config_path).expect("Failed to read configuration file");
        trace!("Parsing: {:?}", config_content);

        match toml::from_str(&config_content) {
            Ok(config) => {
                trace!("Parsed config: {:?}", config);
                Ok(config)
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        debug!("Config file not found. Using default config.");
        Ok(ConfigFile::default())
    }
}
