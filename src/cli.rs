use clap::Parser;

#[derive(Parser)]
#[command(name = "wait")]
#[command(version, about = "Waits for specified duration")]
pub struct Cli {
    /// Duration to wait (e.g. "5s", "2m", "1h")
    #[arg(default_value = "25m")]
    pub duration: String,
}
