mod cli;
mod client;
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

use env_logger::Env;
use log::info;

/// The main entry point of the application.
///
/// This function initializes the logger, parses command line arguments,
/// and starts the focus timer.
/// The main logic is implemented in the [`focus`](crate::focus) module.
/// The [`Cli`] struct is used to parse command line arguments.
/// With the parsed arguments, a new focus timer is created with [`focus::new`](crate::focus::new).
/// Finally, the focus timer is started with [`focus::Focus::run`](crate::focus::Focus::run).
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation.
///
/// # Errors
///
/// This function will return an error if the focus timer fails to run.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Log the start of command line argument parsing
    info!("Parsing command line arguments");

    // Parse command line arguments
    let args = Cli::parse();

    // Initialize the logger with the specified log level from the command line arguments
    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level)).init();

    // Log the start of the focus timer
    info!("Starting focus timer");

    // Handle subcommands or start the focus timer
    if let Some(command) = args.command {
        client::handle_command(command).await?;
    } else {
        focus::new(args)?.run().await?;
    }

    Ok(())
}
