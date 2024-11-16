mod cli;
mod swaync;
mod swaync_interface;
mod notification;
mod notification_interface;
mod sway_ipc_interface;
mod focus;


use clap::Parser;
use cli::{parse_duration, Cli};

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let duration = if let Some(duration) = parse_duration(&args.duration) {
        println!("Setting DND for {:?}", duration);
        duration
    } else {
        eprintln!("Invalid duration format. Use <number><unit> where unit is s, m, or h");
        std::process::exit(1);
    };

    focus::run(duration).await?;

    Ok(())
}
