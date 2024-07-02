use std::io::{stdout, Write};
use std::process;
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::Parser;

/// A timer program that supports both ascending and descending formats.
#[derive(Parser, Debug)]
#[command(author="Lukas Karafiat")]
struct Options {
    /// Print the time in ascending format.
    #[arg(short = 'a', long = "ascending")]
    print_ascending_time: bool,

    /// Print the time in descending format.
    #[arg(short = 'd', long = "descending")]
    print_descending_time: bool,

    /// Timer durations in the format NUMBER[UNIT] (e.g., 10s, 5m).
    #[arg(value_name = "NUMBER[UNIT]", required = true)]
    times: Vec<String>,
}

#[derive(Debug)]
enum ParsingError {
    InvalidNumber,
    InvalidUnit,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidNumber => write!(f, "Error: Invalid number format"),
            ParsingError::InvalidUnit => write!(f, "Error: Invalid unit format"),
        }
    }
}

impl std::error::Error for ParsingError {}

/// Parses a vector of strings representing time durations and returns the total duration.
/// Each string should be in the format NUMBER[UNIT], where UNIT can be ms, s, m, h, or d.
#[inline]
fn parse_duration(arguments: Vec<String>) -> Result<Duration, ParsingError> {
    let mut duration = Duration::new(0, 0);

    for argument in arguments {
        let (value, unit) = if let Some(index) = argument.find(char::is_alphabetic) {
            (&argument[0..index], &argument[index..])
        } else {
            (&argument[..], "s")
        };

        let number = if let Ok(number) = value.parse::<f64>() {
            number
        } else {
            return Err(ParsingError::InvalidNumber);
        };

        duration += match unit {
            "ms" => Duration::from_secs_f64(number / 1000.0),
            "s" => Duration::from_secs_f64(number),
            "m" => Duration::from_secs_f64(number * 60.0),
            "h" => Duration::from_secs_f64(number * 60.0 * 60.0),
            "d" => Duration::from_secs_f64(number * 60.0 * 60.0 * 24.0),
            _ => return Err(ParsingError::InvalidUnit),
        };
    }

    Ok(duration)
}

/// Formats a `Duration` into a human-readable string.
fn format_duration(seconds: Duration) -> String {
    let mut remaining_seconds = seconds.as_secs();
    let mut remaining_milliseconds = seconds.as_millis();

    let days = remaining_seconds / (60 * 60 * 24);
    remaining_seconds %= 60 * 60 * 24;

    let hours = remaining_seconds / (60 * 60);
    remaining_seconds %= 60 * 60;

    let minutes = remaining_seconds / 60;
    remaining_seconds %= 60;

    let seconds = remaining_seconds;

    remaining_milliseconds %= 1000;
    let milliseconds = remaining_milliseconds;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }

    parts.push(format!("{hours:02}h {minutes:02}m {seconds:02}s {milliseconds:03}ms"));

    parts.join(" ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();

    let sleep_duration = match parse_duration(options.times) {
        Ok(duration) => duration,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let start = Instant::now();
    let tick = Duration::from_millis(10);

    loop {
        let elapsed = start.elapsed();

        if elapsed >= sleep_duration {
            break;
        }

        print!("\x1b[2K\r");

        if options.print_ascending_time {
            print!("{}", format_duration(elapsed));
        }

        if options.print_ascending_time && options.print_descending_time {
            print!(" | ");
        }
        if options.print_descending_time {
            print!("{}", format_duration(sleep_duration - elapsed));
        }

        stdout().flush()?;
        sleep(tick);
    }

    print!("\x1b[2K\r");
    println!("{}", format_duration(sleep_duration));

    Ok(())
}
