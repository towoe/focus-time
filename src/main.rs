mod cli;

use clap::Parser;
use cli::{parse_duration, Cli};

use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use tokio::time::sleep;
use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.erikreider.swaync.cc",
    default_service = "org.erikreider.swaync.cc",
    default_path = "/org/erikreider/swaync/cc"
)]
trait SwayNC {
    async fn set_dnd(&self, state: &bool) -> Result<()>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let connection = Connection::session().await?;
    let proxy = SwayNCProxy::new(&connection).await?;

    proxy.set_dnd(&true).await?;

    let duration = if let Some(duration) = parse_duration(&args.duration) {
        println!("Setting DND for {:?}", duration);
        duration
    } else {
        eprintln!("Invalid duration format. Use <number><unit> where unit is s, m, or h");
        std::process::exit(1);
    };

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
    Ok(())
}
