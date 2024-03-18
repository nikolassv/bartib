use crate::data::activity::Activity;

use chrono::{Datelike, Duration, NaiveDate};

pub struct Filters {}

impl Filters {
    pub fn active(activity: &&Activity) -> bool {
        !activity.is_stopped()
    }
    pub fn today(today: NaiveDate) -> impl Fn(&&Activity) -> bool {
        move |activity: &&Activity| activity.start.date() == today
    }
    pub fn current_week(today: NaiveDate) -> impl Fn(&&Activity) -> bool {
        let from_date = today - Duration::days(i64::from(today.weekday().num_days_from_monday()));
        let to_date = today;
        move |activity: &&Activity| {
            activity.start.date() >= from_date && activity.start.date() <= to_date
        }
    }
    pub fn current_month(today: NaiveDate) -> impl Fn(&&Activity) -> bool {
        let from_date = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
        let to_date = today;
        move |activity: &&Activity| {
            activity.start.date() >= from_date && activity.start.date() <= to_date
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use crate::data::activity;

    use super::*;

    #[test]
    fn filter_active() {
        let activities = data();
        let res: Vec<&Activity> = activities.iter().filter(Filters::active).collect();
        assert_eq!(res.len(), 1);
        assert_eq!(res.first().unwrap().description.as_str(), "d4");
    }

    #[test]
    fn filter_today() {
        let now = date(2024, 3, 19);
        let activities = data();
        let res: Vec<&Activity> = activities
            .iter()
            .filter(Filters::today(now.date()))
            .collect();
        assert_eq!(res.len(), 2);
        assert_eq!(res.first().unwrap().description.as_str(), "d3");
    }

    #[test]
    fn filter_current_week() {
        let now = date(2024, 3, 19);
        let activities = data();
        let res: Vec<&Activity> = activities
            .iter()
            .filter(Filters::current_week(now.date()))
            .collect();
        assert_eq!(res.len(), 3);
        assert_eq!(res.first().unwrap().description.as_str(), "d2");
    }

    #[test]
    fn filter_current_month() {
        let now = date(2024, 3, 19);
        let activities = data();
        let res: Vec<&Activity> = activities
            .iter()
            .filter(Filters::current_month(now.date()))
            .collect();
        assert_eq!(res.len(), 4);
        assert_eq!(res.first().unwrap().description.as_str(), "d1");
    }

    fn data() -> Vec<Activity> {
        let a0 = activity::Activity {
            project: "p1".to_string(),
            description: "d0".to_string(),
            start: date(2024, 2, 11),
            end: Some(date(2024, 2, 11) + Duration::hours(2)),
        };
        let a1 = activity::Activity {
            project: "p1".to_string(),
            description: "d1".to_string(),
            start: date(2024, 3, 11),
            end: Some(date(2024, 3, 11) + Duration::hours(2)),
        };
        let a2 = activity::Activity {
            project: "p1".to_string(),
            description: "d2".to_string(),
            start: date(2024, 3, 18),
            end: Some(date(2024, 3, 18) + Duration::hours(2)),
        };
        let a3 = activity::Activity {
            project: "p1".to_string(),
            description: "d3".to_string(),
            start: date(2024, 3, 19),
            end: Some(date(2024, 3, 19) + Duration::hours(2)),
        };
        let a4 = activity::Activity {
            project: "p1".to_string(),
            description: "d4".to_string(),
            start: date(2024, 3, 19),
            end: None,
        };
        return vec![a0, a1, a2, a3, a4];
    }

    fn date(year: i32, month: u32, day: u32) -> NaiveDateTime {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        return NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap());
    }
}
