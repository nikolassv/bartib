// Utility functions for rounding datetimes.
// Limitations:
// - Cannot handle days properly.
// - Does not consider DST changes, so rounding to hours may be off by an hour.
// - Does not consider leap seconds.
pub fn round_datetime(
    datetime: &chrono::NaiveDateTime,
    round: &chrono::Duration,
) -> chrono::NaiveDateTime {
    let timestamp = datetime.timestamp();
    let round_seconds = round.num_seconds();

    let rounded_timestamp =
        (timestamp as f64 / round_seconds as f64).round() as i64 * round_seconds;

    chrono::NaiveDateTime::from_timestamp_opt(rounded_timestamp, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate};

    use super::*;

    fn fake_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()
    }

    #[test]
    fn test_round_to_5_minutes() {
        let round = Duration::minutes(5);

        assert_eq!(
            round_datetime(&fake_date().and_hms_opt(13, 32, 30).unwrap(), &round),
            fake_date().and_hms_opt(13, 35, 0).unwrap()
        );

        assert_eq!(
            round_datetime(&fake_date().and_hms_opt(13, 31, 1).unwrap(), &round),
            fake_date().and_hms_opt(13, 30, 0).unwrap()
        );
    }

    #[test]
    fn test_round_to_8_hours() {
        let round = Duration::hours(8);

        assert_eq!(
            round_datetime(&fake_date().and_hms_opt(4, 0, 1).unwrap(), &round),
            fake_date().and_hms_opt(8, 0, 0).unwrap()
        );

        assert_eq!(
            round_datetime(&fake_date().and_hms_opt(3, 59, 59).unwrap(), &round),
            fake_date().and_hms_opt(0, 0, 0).unwrap()
        );
    }

    #[test]
    fn test_round_middle_rounds_up() {
        let round = Duration::minutes(10);

        assert_eq!(
            round_datetime(&fake_date().and_hms_opt(13, 5, 0).unwrap(), &round),
            fake_date().and_hms_opt(13, 10, 0).unwrap()
        )
    }
}
