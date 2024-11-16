mod cli;
mod swaync;
mod swaync_interface;
mod notification;
mod notification_interface;
mod sway_ipc_interface;


use clap::Parser;
use cli::{parse_duration, Cli};

use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::{zvariant::Value};

use anyhow::Result;

use swaync_interface::SwayNCInterface;
use notification_interface::NotificationInterface;
use sway_ipc_interface::SwayIpcInterface;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let swaync = SwayNCInterface::new().await?;
    let notify = NotificationInterface::new().await?;
    let mut sway = SwayIpcInterface::new().await?;

    swaync.enable_dnd().await?;

    let duration = if let Some(duration) = parse_duration(&args.duration) {
        println!("Setting DND for {:?}", duration);
        duration
    } else {
        eprintln!("Invalid duration format. Use <number><unit> where unit is s, m, or h");
        std::process::exit(1);
    };

    sway.set_bar_mode_invisible().await?;

    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let tx_clone = Arc::clone(&tx);
    ctrlc::set_handler(move || {
        let mut tx_lock = tx_clone.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(());
        }
    })
    .expect("Error setting Ctrl+C handler");

    tokio::select! {
        _ = sleep(duration) => {},
        _ = rx => {
            println!("\nReceived Ctrl+C, starting cleanup...");
        },
    }

    // Restore the tools and notify the user
    swaync.disable_dnd().await?;
    sway.set_bar_mode_dock().await?;
    
    let mut hints = HashMap::new();
    hints.insert("urgency", &Value::U8(2));

    let _ = notify.notify(
        "Focus time over",
        &format!("{:?} have passed", duration),
        hints,
    ).await?;

    Ok(())
}
