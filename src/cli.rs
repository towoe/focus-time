use clap::Parser;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "wait")]
#[command(version, about = "Waits for specified duration")]
pub struct Cli {
    /// Duration to wait (e.g. "5s", "2m", "1h")
    #[arg(default_value = "25m")]
    pub duration: String,
}

pub fn parse_duration(input: &str) -> Option<Duration> {
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
