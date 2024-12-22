use clap::Parser;

/// Command line interface for the wait command
#[derive(Parser)]
#[command(
    version,
    about = "Create distraction free environment for a limited time."
)]
pub struct Cli {
    /// Duration to wait (e.g. "5s", "2m", "1h")
    pub duration: Option<String>,

    /// Disable notifications
    #[arg(short = 'n', long)]
    pub no_notification: bool,

    /// Keep the status bar visible
    #[arg(short = 's', long)]
    pub keep_status_bar: bool,

    /// Print the remaining time
    #[arg(short = 'p', long)]
    pub print_time: bool,

    /// Path to the configuration file
    #[arg(short = 'c', long)]
    pub config: Option<String>,

    /// Log level (e.g. "trace", "debug", "info", "warn", "error")
    #[arg(short, long, default_value = "error")]
    pub log_level: String,
}
