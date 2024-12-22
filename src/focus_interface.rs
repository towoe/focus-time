use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use zbus::interface;

use crate::focus::AbortSignal;
use crate::timer::Timer;

/// Represents the focus time with a timer and a channel for abort signals.
pub struct FocusTime {
    /// The timer for the focus session.
    pub timer: Timer,
    /// A thread-safe optional sender for abort signals.
    pub tx: Arc<Mutex<Option<oneshot::Sender<AbortSignal>>>>,
}

#[interface(name = "org.towoe.FocusTime")]
impl FocusTime {
    /// Retrieves the remaining time of the focus session as a formatted string.
    ///
    /// # Returns
    ///
    /// A `String` representing the remaining time in a fixed format.
    pub async fn get_remaining_time(&self) -> String {
        self.timer.remaining_str_fixed_format()
    }

    /// Stops the focus timer by sending an abort signal.
    ///
    /// This method locks the mutex, takes the sender if available, and sends an `AbortSignal::Dbus`.
    pub async fn stop_timer(&self) {
        let mut tx_lock = self.tx.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(AbortSignal::Dbus);
        }
    }
}
