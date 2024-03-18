use crate::data::activity::Activity;

use chrono::{Datelike, Duration, Local, NaiveDate};

pub struct Filters {}

impl Filters {
    pub fn active(activity: &&Activity) -> bool {
        !activity.is_stopped()
    }
    pub fn today() -> impl Fn(&&Activity) -> bool {
        let today = Local::now().naive_local().date();
        move |activity: &&Activity| activity.start.date() == today
    }
    pub fn current_week() -> impl Fn(&&Activity) -> bool {
        let today = Local::now().naive_local().date();
        let from_date = today - Duration::days(i64::from(today.weekday().num_days_from_monday()));
        let to_date = today;
        move |activity: &&Activity| {
            activity.start.date() >= from_date && activity.start.date() <= to_date
        }
    }
    pub fn current_month() -> impl Fn(&&Activity) -> bool {
        let today = Local::now().naive_local().date();
        let from_date = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
        let to_date = today;
        move |activity: &&Activity| {
            activity.start.date() >= from_date && activity.start.date() <= to_date
        }
    }
}
