mod cli;
mod swaync;
mod notification;

use swaync::SwayNCProxy;

use clap::Parser;
use cli::{parse_duration, Cli};

use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::{zvariant::Value, Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let connection = Connection::session().await?;
    let proxy = SwayNCProxy::new(&connection).await?;

    let connection_notifications = Connection::session().await?;
    let proxy_notifications = notification::NotificationsProxy::new(&connection_notifications).await?;

    let mut connection_sway = swayipc_async::Connection::new().await.expect("Error connecting to sway");

    proxy.set_dnd(&true).await?;

    let duration = if let Some(duration) = parse_duration(&args.duration) {
        println!("Setting DND for {:?}", duration);
        duration
    } else {
        eprintln!("Invalid duration format. Use <number><unit> where unit is s, m, or h");
        std::process::exit(1);
    };

    connection_sway.run_command("bar mode invisible").await.expect("Error setting sway bar mode");

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

    // unset Dnd
    proxy.set_dnd(&false).await?;
    connection_sway.run_command("bar mode dock").await.expect("Error setting sway bar mode");
    let mut hints = HashMap::new();
    hints.insert("urgency", &Value::U8(2));

    let _ = proxy_notifications.notify(
        "focus-time",
        0,
        "selection-mode",
        "Focus time over", format!("{:?} have passed", duration).as_str(),
        &[],
        hints,
        0,
    ).await?;

    Ok(())
}
