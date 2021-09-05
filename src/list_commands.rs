use anyhow::Result;
use chrono::{naive, NaiveDate};

use crate::activity;
use crate::bartib_file;
use crate::output;

pub struct ActivityFilter {
    pub number_of_activities: Option<usize>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub date: Option<NaiveDate>,
}

// lists all currently runninng activities.
pub fn list_running(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let running_activities = get_running_activities(&file_content);
    output::list_running_activities(&running_activities);

    Ok(())
}

// lists tracked activities
//
// the activities will be ordered chronologically.
pub fn list(file_name: &str, filter: ActivityFilter, do_group_activities: bool) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities = get_activities(&file_content);
    let mut filtered_activities: Vec<&activity::Activity> =
        filter_activities(activities, &filter).collect();

    filtered_activities.sort_by_key(|activity| activity.start);

    let first_element =
        get_index_of_first_element(filtered_activities.len(), filter.number_of_activities);

    if do_group_activities {
        output::list_activities_grouped_by_date(
            &filtered_activities[first_element..filtered_activities.len()],
        );
    } else {
        let with_start_dates = !filter.date.is_some();
        output::list_activities(
            &filtered_activities[first_element..filtered_activities.len()],
            with_start_dates,
        );
    }

    Ok(())
}

// lists all projects
pub fn list_projects(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let mut all_projects: Vec<&String> = get_activities(&file_content)
        .map(|activity| &activity.project)
        .collect();

    all_projects.sort_unstable();
    all_projects.dedup();

    for project in all_projects {
        println!("\"{}\"", project);
    }

    Ok(())
}

// return last finished activity
pub fn display_last_activity(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let last_activity = get_last_activity_by_end(&file_content);

    if let Some(activity) = last_activity {
        output::display_single_activity(&activity);
    } else {
        println!("No activity has been finished yet.")
    }

    Ok(())
}

fn get_index_of_first_element(length: usize, sub: Option<usize>) -> usize {
    if let Some(s) = sub {
        length.saturating_sub(s)
    } else {
        0
    }
}

fn get_running_activities(file_content: &[bartib_file::Line]) -> Vec<&activity::Activity> {
    get_activities(file_content)
        .filter(|activity| !activity.is_stopped())
        .collect()
}

fn get_activities(file_content: &[bartib_file::Line]) -> impl Iterator<Item = &activity::Activity> {
    file_content
        .iter()
        .map(|line| line.activity.as_ref())
        .filter_map(|activity_result| activity_result.ok())
}

fn filter_activities<'a>(
    activities: impl Iterator<Item = &'a activity::Activity>,
    filter: &ActivityFilter,
) -> impl Iterator<Item = &'a activity::Activity> {
    let from_date: NaiveDate;
    let to_date: NaiveDate;

    if let Some(date) = filter.date {
        from_date = date;
        to_date = date;
    } else {
        from_date = filter.from_date.unwrap_or(naive::MIN_DATE);
        to_date = filter.to_date.unwrap_or(naive::MAX_DATE);
    }

    activities.filter(move |activity| {
        activity.start.date() >= from_date && activity.start.date() <= to_date
    })
}

pub fn get_last_activity_by_end(file_content: &Vec<bartib_file::Line>) -> Option<&activity::Activity> {
    get_activities(&file_content)
        .filter(|activity| activity.is_stopped())
        .max_by_key(|activity| activity.end.unwrap_or(naive::MIN_DATE.and_hms(0, 0, 0)))
}

pub fn get_last_activity_by_start(file_content: &Vec<bartib_file::Line>) -> Option<&activity::Activity> {
    get_activities(&file_content).max_by_key(|activity| activity.start)
}