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
async fn main() -> Result<()> {
    info!("Parsing command line arguments");
    let args = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level)).init();

    info!("Loading file config");
    let file_config = config::load_from_file(&args.config).expect("Could not load file config.");

    info!("Creating focus timer configuration");
    let config = focus::create_config(file_config, args);

    info!("Starting focus timer");
    focus::new(config).run().await?;

    Ok(())
}
