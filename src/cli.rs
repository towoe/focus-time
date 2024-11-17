use clap::Parser;

/// Command line interface for the wait command
#[derive(Parser)]
#[command(name = "wait")]
#[command(version, about = "Waits for specified duration")]
pub struct Cli {
    /// Duration to wait (e.g. "5s", "2m", "1h")
    #[arg(default_value = "25m")]
    pub duration: String,

    /// Disable notifications
    #[arg(short = 'n', long)]
    pub no_notification: bool,

    /// Keep the status bar visible
    #[arg(short = 's', long)]
    pub keep_status_bar: bool,

    // Print the remaining time
    #[arg(short = 'p', long)]
    pub print_time: bool,

    /// Log level (e.g. "info", "debug", "warn", "error")
    #[arg(short, long, default_value = "error")]
    pub log_level: String,
}
