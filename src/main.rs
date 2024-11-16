mod cli;
mod config;
mod focus;
mod notification;
mod notification_interface;
mod sway_ipc_interface;
mod swaync;
mod swaync_interface;

use clap::Parser;
use cli::Cli;
use config::Config;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let config = Config::set(&args);

    focus::run(config).await?;

    Ok(())
}
