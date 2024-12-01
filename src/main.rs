mod cli;
mod config;
mod focus;
mod focus_interface;
mod notification;
mod notification_interface;
mod sway_ipc_interface;
mod swaync;
mod swaync_interface;
mod timer;

use clap::Parser;
use cli::Cli;

use anyhow::Result;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Parsing command line arguments");
    let args = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level)).init();

    info!("Starting focus timer");
    focus::new(args)?.run().await?;

    Ok(())
}
