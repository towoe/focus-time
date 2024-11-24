use std::io::Write;
use std::time::{Duration, Instant};

use log::debug;

#[derive(Copy, Clone)]
pub struct Timer {
    duration: Duration,
    start: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: Instant::now(),
        }
    }

    pub fn is_remaining(&self) -> bool {
        self.remaining() > Duration::from_secs(1)
    }

    pub fn remaining(&self) -> Duration {
        // If the timer is already in the past, the current time is higher
        if (self.start + self.duration) < Instant::now() {
            Duration::from_secs(0)
        } else {
            self.duration - self.start.elapsed()
        }
    }

    pub fn remaining_time_parts(&self) -> (u64, u64, u64) {
        let remaining = self.remaining();
        (
            remaining.as_secs() / 3600,
            (remaining.as_secs() / 60) % 60,
            remaining.as_secs() % 60,
        )
    }

    pub fn timer_time_parts(&self) -> (u64, u64, u64) {
        let duration = self.duration;
        (
            duration.as_secs() / 3600,
            (duration.as_secs() / 60) % 60,
            duration.as_secs() % 60,
        )
    }

    pub fn remaining_str_fixed_format(&self) -> String {
        let (h, m, s) = self.remaining_time_parts();
        debug!("Remaining time: {}:{}:{}", h, m, s);
        format!("{:02}:{:02}:{:02}", h, m, s)
    }

    pub fn remaining_str_adapted_format(&self) -> String {
        let (h, m, s) = self.remaining_time_parts();
        debug!("Remaining time: {}:{}:{}", h, m, s);
        match (h, m, s) {
            (1.., _, _) => format!("{:02}:{:02}:{:02}", h, m, s),
            (0, 1.., _) => format!("{:02}:{:02}", m, s - s % 30),
            (0, 0, 10..) => format!("{:02}:{:02}", m, s - s % 10),
            (0, 0, _) => format!("{:02}:{:02}", 0, s),
        }
    }
}

impl std::fmt::Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (th, tm, ts) = self.timer_time_parts();
        let (h, m, s) = self.remaining_time_parts();
        write!(
            f,
            "{:02}:{:02}:{:02} [{:02}:{:02}:{:02}]",
            h, m, s, th, tm, ts
        )
    }
}

/// Displays a countdown timer in the terminal.
///
/// This function shows the remaining time in HH:MM:SS format when hours are present,
/// or MM:SS format for shorter durations. The display updates with different frequencies
/// depending on the remaining time:
/// - Hours remaining: updates every 60 seconds
/// - Minutes remaining: updates every 10 seconds
/// - Last minute: updates every second
///
/// The cursor is hidden during the countdown and restored when finished.
pub async fn print_remaining_time(timer: Timer) {
    print!("\x1B[?25l"); // Hide cursor
    while timer.is_remaining() {
        print!(
            "\x1B[2K\rTime remaining: {}",
            timer.remaining_str_adapted_format()
        );
        std::io::stdout().flush().unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    print!("\x1B[?25h"); // Show cursor
}
