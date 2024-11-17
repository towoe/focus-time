use std::io::Write;

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
/// * `duration` - The total duration to count down from
///
/// # Examples
///
/// ```rust
/// let duration = std::time::Duration::from_secs(3600); // 1 hour
/// print_remaining_time(duration).await;
/// ```
pub async fn print_remaining_time(duration: std::time::Duration) {
    let start = std::time::Instant::now();
    print!("\x1B[?25l"); // Hide cursor
    while start.elapsed() < duration {
        let remaining = duration - start.elapsed();
        let seconds = remaining.as_secs() % 60;
        let minutes = (remaining.as_secs() / 60) % 60;
        let hours = remaining.as_secs() / 3600;
        match (hours, minutes, seconds) {
            (1.., _, _) => {
                print!("\x1B[2K\rTime remaining: {:02}:{:02}:{:02}", hours, minutes, 0);
            }
            (0, 1.., _) => print!("\x1B[2K\rTime remaining: {:02}:{:02}", minutes, 0),
            (0, 0, 10..) => {
                print!(
                    "\x1B[2K\rTime remaining: {:02}:{:02}",
                    minutes,
                    seconds - seconds % 10
                );
            }
            (0, 0, _) => {
                print!("\x1B[2K\rTime remaining: {:02}:{:02}", minutes, seconds);
            }
        }
        std::io::stdout().flush().unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    print!("\x1B[?25h"); // Show cursor
}
