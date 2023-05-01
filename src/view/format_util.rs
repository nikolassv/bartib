use chrono::Duration;
use std::str::FromStr;

pub enum Format {
    SHELL,
    JSON,
}

impl Default for Format {
    fn default() -> Self {
        Format::SHELL
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shell" => Ok(Format::SHELL),
            "json" => Ok(Format::JSON),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

pub fn format_duration(duration: &Duration) -> String {
    let mut duration_string = String::new();

    if duration.num_hours() > 0 {
        duration_string.push_str(&format!("{}h ", duration.num_hours()));
    }

    if duration.num_minutes() > 0 {
        duration_string.push_str(&format!("{:0>2}m", duration.num_minutes() % 60));
    } else {
        duration_string.push_str(&format!("{}s", duration.num_seconds() % 60));
    }

    duration_string
}
