use std::io::Write;
use std::time::{Duration, Instant};

use log::debug;

/// A simple timer struct that tracks a duration and start time.
#[derive(Copy, Clone)]
pub struct Timer {
    duration: Duration,
    start: Instant,
}

impl Timer {
    /// Creates a new `Timer` with the specified duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration for the timer.
    ///
    /// # Returns
    ///
    /// A new `Timer` instance.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: Instant::now(),
        }
    }

    /// Checks if there is more than one second remaining on the timer.
    ///
    /// # Returns
    ///
    /// `true` if more than one second is remaining, `false` otherwise.
    pub fn is_remaining(&self) -> bool {
        self.remaining() > Duration::from_secs(1)
    }

    /// Calculates the remaining time on the timer.
    ///
    /// # Returns
    ///
    /// The remaining duration.
    pub fn remaining(&self) -> Duration {
        // If the timer is already in the past, the current time is higher
        if (self.start + self.duration) < Instant::now() {
            Duration::from_secs(0)
        } else {
            self.duration - self.start.elapsed()
        }
    }

    /// Gets the remaining time in hours, minutes, and seconds.
    ///
    /// # Returns
    ///
    /// A tuple containing the hours, minutes, and seconds remaining.
    pub fn remaining_time_parts(&self) -> (u64, u64, u64) {
        let remaining = self.remaining();
        (
            remaining.as_secs() / 3600,
            (remaining.as_secs() / 60) % 60,
            remaining.as_secs() % 60,
        )
    }

    /// Gets the total timer duration in hours, minutes, and seconds.
    ///
    /// # Returns
    ///
    /// A tuple containing the hours, minutes, and seconds of the timer duration.
    pub fn timer_time_parts(&self) -> (u64, u64, u64) {
        let duration = self.duration;
        (
            duration.as_secs() / 3600,
            (duration.as_secs() / 60) % 60,
            duration.as_secs() % 60,
        )
    }

    /// Formats the remaining time as a fixed format string (HH:MM:SS).
    ///
    /// # Returns
    ///
    /// A string representing the remaining time in HH:MM:SS format.
    pub fn remaining_str_fixed_format(&self) -> String {
        let (h, m, s) = self.remaining_time_parts();
        debug!("Remaining time: {}:{}:{}", h, m, s);
        format!("{:02}:{:02}:{:02}", h, m, s)
    }

    /// Formats the remaining time as an adapted format string.
    ///
    /// The format adapts based on the remaining time:
    /// - Hours remaining: HH:MM:SS
    /// - Minutes remaining: MM:SS (rounded to the nearest 30 seconds)
    /// - Last minute: MM:SS (rounded to the nearest 10 seconds)
    ///
    /// # Returns
    ///
    /// A string representing the remaining time in an adapted format.
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
    /// Formats the timer for display.
    ///
    /// The format includes both the total timer duration and the remaining time.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure.
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
///
/// # Arguments
///
/// * `timer` - The timer to display.
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
