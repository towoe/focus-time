use crate::config::Config;
use crate::notification_interface::NotificationInterface;
use crate::sway_ipc_interface::SwayIpcInterface;
use crate::swaync_interface::SwayNCInterface;

use anyhow::Result;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::zvariant::Value;

pub async fn run(config: Config) -> Result<()> {
    // Initialize the interfaces
    let swaync = SwayNCInterface::new().await?;
    let mut sway = SwayIpcInterface::new().await?;

    // Open channels to communicate with the Ctrl+C handler
    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // Set the tools to the desired state
    swaync.enable_dnd().await?;
    if !config.keep_status_bar {
        sway.set_bar_mode_invisible().await?;
    }

    // Set the Ctrl+C handler
    // This is needed so the program does not just end on Ctrl+c, but clean up
    // before. For example, unset the do not disturb state.
    let tx_clone = Arc::clone(&tx);
    ctrlc::set_handler(move || {
        let mut tx_lock = tx_clone.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(());
        }
    })
    .expect("Error setting Ctrl+C handler");

    // Wait for the `duration` specified time or a Ctrl+C signal
    tokio::select! {
        _ = sleep(config.duration) => {},
        _ = rx => {
            println!("\nReceived Ctrl+C, starting cleanup...");
        },
    }

    // Restore the tools and notify the user
    swaync.disable_dnd().await?;

    if !config.keep_status_bar {
        sway.set_bar_mode_dock().await?;
    }

    let mut hints = HashMap::new();
    hints.insert("urgency", &Value::U8(2));

    if !config.no_notification {
        let notify = NotificationInterface::new().await?;
        let _ = notify
            .notify(
                "Focus time over",
                &format!("{:?} have passed", config.duration),
                hints,
            )
            .await?;
    }

    Ok(())
}
