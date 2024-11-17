use std::time::{Duration, Instant};

use zbus::interface;

pub struct FocusTime {
    pub duration: Duration,
    pub start: Instant,
}

#[interface(name = "org.towoe.FocusTime")]
impl FocusTime {
    pub async fn get_remaining_time(&self) -> String {
        let remaining = self.duration - self.start.elapsed();
        let (h, m, s) = (
            remaining.as_secs() / 3600,
            (remaining.as_secs() / 60) % 60,
            remaining.as_secs() % 60,
        );
        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}
