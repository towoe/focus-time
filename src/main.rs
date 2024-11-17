mod cli;
mod config;
mod display;
mod focus;
mod focus_interface;
mod notification;
mod notification_interface;
mod sway_ipc_interface;
mod swaync;
mod swaync_interface;

use clap::Parser;
use cli::Cli;
use config::Config;

use anyhow::Result;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level)).init();
    info!("Starting focus timer");

    let config = Config::set(&args).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    focus::run(config).await?;

    Ok(())
}
