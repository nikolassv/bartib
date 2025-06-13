use chrono::Duration;

pub fn format_duration(duration: &Duration) -> String {
    let mut duration_string = String::new();

    if duration.num_hours() > 0 {
        duration_string.push_str(&format!("{}h ", duration.num_hours()));
    }

    if duration.num_minutes() > 0 {
        duration_string.push_str(&format!("{:0>2}m", duration.num_minutes() % 60));
    } else {
        #[cfg(not(feature = "second-precision"))]
        duration_string.push_str("<1m");
        #[cfg(feature = "second-precision")]
        duration_string.push_str(&format!("{:0>2}s", duration.num_seconds() % 60));
    }

    duration_string
}
