use chrono::NaiveDate;
use std::collections::BTreeMap;

use crate::activity;
use crate::conf;
use crate::format_util;
use crate::table;

// displays a table with activities
pub fn list_activities(activities: &[&activity::Activity], with_start_dates: bool) {
    if activities.is_empty() {
        println!("No activity to display");
        return;
    }

    let mut activity_table = table::Table::new(vec![
        "   ",
        "Started",
        "Stopped",
        "Description",
        "Project",
        "Duration",
    ]);

    activities
        .iter()
        .map(|t| get_activity_table_row(&t, with_start_dates))
        .for_each(|row| activity_table.add_row(row));

    println!("\n{}", activity_table);
}

// list activities grouped by the dates of their start time
pub fn list_activities_grouped_by_date(activities: &[&activity::Activity]) {
    if activities.is_empty() {
        println!("No activity to display");
        return;
    }

    let activities_by_date = group_activities_by_date(activities);

    for (date, activity_list) in activities_by_date {
        println!("{}", date);
        list_activities(&activity_list, false);
        println!();
    }
}

// displays a table with running activities (no end time)
pub fn list_running_activities(running_activities: &[&activity::Activity]) {
    if running_activities.is_empty() {
        println!("No Activity is currently running");
    } else {
        let mut activity_table =
            table::Table::new(vec!["Started At", "Description", "Project", "Duration"]);

        running_activities
            .iter()
            .map(|activity| {
                table::Row::new(vec![
                    activity.start.format(conf::FORMAT_DATETIME).to_string(),
                    activity.description.clone(),
                    activity.project.to_string(),
                    format_util::format_duration(&activity.get_duration()),
                ])
            })
            .for_each(|row| activity_table.add_row(row));

        println!("\n{}", activity_table);
    }
}

// create a row for a activity
//
// the date of the end is shown when it is not the same date as the start
fn get_activity_table_row(activity: &&activity::Activity, with_start_dates: bool) -> table::Row {
    let display_end = activity.end.map_or_else(
        || "-".to_string(),
        |end| {
            if activity.start.date() == end.date() {
                end.format(conf::FORMAT_TIME).to_string()
            } else {
                end.format(conf::FORMAT_DATETIME).to_string()
            }
        },
    );

    let start_format = if with_start_dates {
        conf::FORMAT_DATETIME
    } else {
        conf::FORMAT_TIME
    };

    table::Row::new(vec![
        if !activity.is_stopped() {
            " * ".to_string()
        } else {
            " ".to_string()
        },
        activity.start.format(start_format).to_string(),
        display_end,
        activity.description.clone(),
        activity.project.to_string(),
        format_util::format_duration(&activity.get_duration()),
    ])
}

// groups activities in vectors of activities that started at the same day
fn group_activities_by_date<'a>(
    activities: &[&'a activity::Activity],
) -> BTreeMap<NaiveDate, Vec<&'a activity::Activity>> {
    let mut activities_by_date = BTreeMap::new();

    for &activity in activities.iter() {
        activities_by_date
            .entry(activity.start.date())
            .or_insert(Vec::new())
            .push(activity);
    }

    for activity_list in activities_by_date.values_mut() {
        activity_list.sort_by_key(|activity| activity.start);
    }

    activities_by_date
}
