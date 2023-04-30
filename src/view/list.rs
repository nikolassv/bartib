use chrono::NaiveDate;
use nu_ansi_term::Color;
use std::collections::BTreeMap;
use serde::Serialize;

use crate::conf;
use crate::data::activity;
use crate::view::format_util;
use crate::view::table;

use super::format_util::Format;

const HOURS_IN_DAY: i64 = 24;
const MINUTES_IN_HOUR: i64 = 60;
const SECONDS_IN_MINUTE: i64 = 60;

#[derive(Serialize)]
pub struct MultiDayListEntry<'a> {
    pub date: NaiveDate,
    pub entries: Vec<&'a activity::Activity>
}

#[derive(Serialize)]
pub struct SerializableDuration {
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl From<&chrono::Duration> for SerializableDuration {
    fn from(value: &chrono::Duration) -> Self {
        Self {
            days: value.num_days(),
            hours: value.num_hours() % HOURS_IN_DAY,
            minutes: value.num_minutes() % MINUTES_IN_HOUR,
            seconds: value.num_seconds() % SECONDS_IN_MINUTE
        }
    }
}


// displays a table with activities
pub fn list_activities(activities: &[&activity::Activity], with_start_dates: bool, format: Format) -> Result<(), serde_json::Error> {
    match format {
        Format::JSON => {
            println!("{}", serde_json::to_string(&activities)?);
            Ok(())
        },
        Format::SHELL => {
            if activities.is_empty() {
                println!("No activity to display");
                return Ok(());
            }
        
            let mut activity_table = create_activity_table();
        
            activities
                .iter()
                .map(|t| get_activity_table_row(t, with_start_dates))
                .for_each(|row| activity_table.add_row(row));
        
            println!("\n{}", activity_table);
            Ok(())
        }
    }
}

// list activities grouped by the dates of their start time
pub fn list_activities_grouped_by_date(activities: &[&activity::Activity], format: Format) -> Result<(), serde_json::Error> {
    match format {
        Format::JSON => {
            println!("{}", serde_json::to_string(&group_activities_by_date(activities)
                .into_iter()
                .map(|(date, entries)| MultiDayListEntry{ date: date, entries: entries })
                .collect::<Vec<_>>())?);
            Ok(())
        },
        Format::SHELL => {
            if activities.is_empty() {
                println!("No activity to display");
                return Ok(());
            }
        
            let mut activity_table = create_activity_table();
        
            group_activities_by_date(activities)
                .iter()
                .map(|(date, activity_list)| {
                    create_activities_group(&format!("{}", date), activity_list.as_slice())
                })
                .for_each(|g| activity_table.add_group(g));
        
            println!("\n{}", activity_table);
            Ok(())
        }
    }
}

fn create_activity_table() -> table::Table {
    table::Table::new(vec![
        table::Column {
            label: "Started".to_string(),
            wrap: table::Wrap::NoWrap,
        },
        table::Column {
            label: "Stopped".to_string(),
            wrap: table::Wrap::NoWrap,
        },
        table::Column {
            label: "Description".to_string(),
            wrap: table::Wrap::Wrap,
        },
        table::Column {
            label: "Project".to_string(),
            wrap: table::Wrap::Wrap,
        },
        table::Column {
            label: "Duration".to_string(),
            wrap: table::Wrap::NoWrap,
        },
    ])
}

fn create_activities_group(title: &str, activities: &[&activity::Activity]) -> table::Group {
    let rows = activities
        .iter()
        .map(|a| get_activity_table_row(a, false))
        .collect();
    table::Group::new(Some(title.to_string()), rows)
}

// displays a table with running activities (no end time)
pub fn list_running_activities(activities: &[&activity::Activity]) {
    if activities.is_empty() {
        println!("No Activity is currently running");
    } else {
        let mut activity_table = table::Table::new(vec![
            table::Column {
                label: "Started At".to_string(),
                wrap: table::Wrap::NoWrap,
            },
            table::Column {
                label: "Description".to_string(),
                wrap: table::Wrap::Wrap,
            },
            table::Column {
                label: "Project".to_string(),
                wrap: table::Wrap::Wrap,
            },
            table::Column {
                label: "Duration".to_string(),
                wrap: table::Wrap::NoWrap,
            },
        ]);

        activities
            .iter()
            .map(|activity| {
                table::Row::new(vec![
                    activity.start.format(conf::FORMAT_DATETIME).to_string(),
                    activity.description.clone(),
                    activity.project.clone(),
                    format_util::format_duration(&activity.get_duration()),
                ])
            })
            .for_each(|row| activity_table.add_row(row));

        println!("\n{}", activity_table);
    }
}

// display a list of projects and descriptions with index number
pub fn list_descriptions_and_projects(descriptions_and_projects: &[(&String, &String)]) {
    if descriptions_and_projects.is_empty() {
        println!("No activities have been tracked yet");
    } else {
        let mut descriptions_and_projects_table = table::Table::new(vec![
            table::Column {
                label: " # ".to_string(),
                wrap: table::Wrap::NoWrap,
            },
            table::Column {
                label: "Description".to_string(),
                wrap: table::Wrap::Wrap,
            },
            table::Column {
                label: "Project".to_string(),
                wrap: table::Wrap::Wrap,
            },
        ]);

        let mut i = descriptions_and_projects.len();

        for (description, project) in descriptions_and_projects {
            i = i.saturating_sub(1);

            descriptions_and_projects_table.add_row(table::Row::new(vec![
                format!("[{}]", i),
                description.to_string(),
                project.to_string(),
            ]));
        }

        println!("\n{}", descriptions_and_projects_table);
    }
}

// create a row for a activity
//
// the date of the end is shown when it is not the same date as the start
fn get_activity_table_row(activity: &activity::Activity, with_start_dates: bool) -> table::Row {
    let more_then_one_day = activity
        .end
        .map_or(false, |end| activity.start.date() != end.date());

    let display_end = activity.end.map_or_else(
        || "-".to_string(),
        |end| {
            if more_then_one_day {
                end.format(conf::FORMAT_DATETIME).to_string()
            } else {
                end.format(conf::FORMAT_TIME).to_string()
            }
        },
    );

    let start_format = if with_start_dates {
        conf::FORMAT_DATETIME
    } else {
        conf::FORMAT_TIME
    };

    let mut new_row = table::Row::new(vec![
        activity.start.format(start_format).to_string(),
        display_end,
        activity.description.clone(),
        activity.project.clone(),
        format_util::format_duration(&activity.get_duration()),
    ]);

    if !activity.is_stopped() {
        new_row.set_color(Color::Green.normal());
    } else if more_then_one_day {
        new_row.set_color(Color::Yellow.normal());
    }

    new_row
}

// groups activities in vectors of activities that started at the same day
fn group_activities_by_date<'a>(
    activities: &[&'a activity::Activity],
) -> BTreeMap<NaiveDate, Vec<&'a activity::Activity>> {
    let mut activities_by_date = BTreeMap::new();

    for &activity in activities.iter() {
        activities_by_date
            .entry(activity.start.date())
            .or_insert_with(Vec::new)
            .push(activity);
    }

    for activity_list in activities_by_date.values_mut() {
        activity_list.sort_by_key(|activity| activity.start);
    }

    activities_by_date
}
