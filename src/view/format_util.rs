use chrono::Duration;

pub fn format_duration(duration: &Duration) -> String {
    let mut duration_string = String::new();

    if duration.num_days() > 0 {
        duration_string.push_str(&format!("{}d ", duration.num_days()));
    }

    if duration.num_hours() > 0 {
        duration_string.push_str(&format!("{}h ", duration.num_hours() % 24));
    }

    if duration.num_minutes() > 0 {
        duration_string.push_str(&format!("{:0>2}m", duration.num_minutes() % 60));
    } else {
        duration_string.push_str(&format!("{}s", duration.num_seconds() % 60));
    }

    duration_string
}
