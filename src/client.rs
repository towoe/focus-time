use crate::cli::Commands;
use anyhow::Result;
use zbus::Connection;

use zbus::proxy;

#[proxy(
    interface = "org.towoe.FocusTime",
    default_service = "org.towoe.FocusTime",
    default_path = "/org/towoe/FocusTime"
)]
pub trait FocusTimer {
    fn get_remaining_time(&self) -> zbus::Result<String>;
    fn get_paused(&self) -> zbus::Result<bool>;
    fn stop_timer(&self) -> zbus::Result<()>;
    fn toggle_pause(&self) -> zbus::Result<()>;
}

pub async fn handle_command(command: Commands) -> Result<()> {
    let connection = Connection::session().await?;
    let proxy = FocusTimerProxy::new(&connection).await?;

    match command {
        Commands::Stop => {
            proxy.stop_timer().await?;
            println!("Focus timer stopped.");
        }
        Commands::TogglePause => {
            proxy.toggle_pause().await?;
            println!("Focus timer toggled pause.");
        }
        Commands::Status => {
            let time = proxy.get_remaining_time().await?;
            let paused = proxy.get_paused().await?;
            if paused {
                println!("{time} (paused)");
            } else {
                println!("{time}");
            }
        }
    }

    Ok(())
}
