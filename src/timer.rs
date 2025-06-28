use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use log::debug;

/// A simple timer struct that tracks a duration and start time.
#[derive(Copy, Clone)]
pub struct Timer {
    duration: Duration,
    start: Instant,
    paused_time: Duration,
    is_paused: bool,
}

impl Timer {
    /// Creates a new `Timer` with the specified duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration for the timer.
    ///
    /// # Returns
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: Instant::now(),
            paused_time: Duration::from_secs(0),
            is_paused: false,
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
        if self.is_paused {
            self.duration.saturating_sub(self.paused_time)
        } else if (self.start + self.duration) < Instant::now() {
            Duration::from_secs(0)
        } else {
            self.duration - self.start.elapsed()
        }
    }

    /// Pauses the timer if it's running.
    pub fn pause(&mut self) {
        if !self.is_paused {
            self.paused_time = self.start.elapsed();
            self.is_paused = true;
        }
    }

    /// Resumes the timer if it's paused.
    pub fn resume(&mut self) {
        if self.is_paused {
            self.start = Instant::now() - self.paused_time;
            self.is_paused = false;
        }
    }

    /// Toggles the timer between paused and running states.
    pub fn toggle_pause(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    /// Returns whether the timer is currently paused.
    pub fn is_paused(&self) -> bool {
        self.is_paused
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
        format!("{h:02}:{m:02}:{s:02}")
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
            (1.., _, _) => format!("{h:02}:{m:02}:{s:02}"),
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
        write!(f, "{h:02}:{m:02}:{s:02} [{th:02}:{tm:02}:{ts:02}]")
    }
}

/// Displays a countdown timer in the terminal with pause support.
///
/// This function shows the remaining time and updates based on pause state.
/// The display updates every second and shows "(PAUSED)" when the timer is paused.
///
/// # Arguments
///
/// * `timer` - Arc<Mutex<Timer>> to display with pause support.
pub async fn print_remaining_time_with_pause(timer: Arc<Mutex<Timer>>) {
    print!("\x1B[?25l"); // Hide cursor
    loop {
        let (remaining, is_paused) = {
            let timer_guard = timer.lock().unwrap();
            if !timer_guard.is_remaining() {
                break;
            }
            (
                timer_guard.remaining_str_adapted_format(),
                timer_guard.is_paused(),
            )
        };

        let status = if is_paused { " (PAUSED)" } else { "" };
        print!("\x1B[2K\rTime remaining: {remaining}{status}");
        std::io::stdout().flush().unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    print!("\x1B[?25h"); // Show cursor
}
