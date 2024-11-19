use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use zbus::interface;

use crate::timer::Timer;

pub struct FocusTime {
    pub timer: Timer,
    pub tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

#[interface(name = "org.towoe.FocusTime")]
impl FocusTime {
    pub async fn get_remaining_time(&self) -> String {
        self.timer.remaining_str_fixed_format()
    }

    pub async fn stop_timer(&self) {
        let mut tx_lock = self.tx.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(());
        }
    }
}
