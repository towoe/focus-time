use clap::Parser;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
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

#[derive(Parser)]
#[command(name = "wait")]
#[command(about = "Waits for specified duration")]
struct Args {
    /// Duration to wait (e.g. "5s", "2m", "1h")
    #[arg(default_value = "25m")]
    duration: String,
}

fn parse_duration(input: &str) -> Option<Duration> {
    if input.is_empty() {
        return None;
    }

    let (number_part, unit_part) = input
        .trim()
        .chars()
        .partition::<String, _>(|c| c.is_ascii_digit());

    let value: u64 = match number_part.parse() {
        Ok(num) => num,
        Err(_) => return None,
    };

    match unit_part.as_str() {
        "s" => Some(Duration::from_secs(value)),
        "m" => Some(Duration::from_secs(value * 60)),
        "h" => Some(Duration::from_secs(value * 60 * 60)),
        "d" => Some(Duration::from_secs(value * 60 * 60 * 24)),
        _ => {
            println!("{unit_part}");
            None
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

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
