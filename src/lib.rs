use anyhow::{anyhow, bail, Context, Result, Error};
use chrono::{naive, NaiveDate};

use crate::bartib_file::Line;
use crate::activity::Activity;
use std::process::Command;

mod activity;
pub mod bartib_file;
pub mod conf;
mod format_util;
mod output;
mod table;

pub struct ActivityFilter {
    pub number_of_activities: Option<usize>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub date: Option<NaiveDate>,
}

// starts a new activity
pub fn start(file_name: &str, project_name: &str, activity_description: &str) -> Result<()>{
    let mut file_content: Vec<Line> = Vec::new();

    if let Ok(mut previous_file_content) = bartib_file::get_file_content(file_name) {
        // if we start a new activities programaticly, we stop all other activities first.
        // However, we must not assume that there is always only one activity
        // running as the user may have started activities manually
        stop_all_running_activities(&mut previous_file_content);

        file_content.append(&mut previous_file_content);
    }

    let activity = activity::Activity::start(project_name.to_string(), activity_description.to_string());

    save_new_activity(file_name, &mut file_content, activity)
}

fn save_new_activity(file_name: &str, file_content: &mut Vec<Line>, activity: Activity) -> Result<(), Error> {
    println!(
        "Started activity: \"{}\" ({}) at {}",
        activity.description,
        activity.project,
        activity.start.format(conf::FORMAT_DATETIME)
    );

    file_content.push(bartib_file::Line::for_activity(activity));
    bartib_file::write_to_file(file_name, &file_content).context(format!("Could not write to file: {}", file_name))
}

// stops all currently running activities
pub fn stop(file_name: &str) -> Result<()>{
    let mut file_content = bartib_file::get_file_content(file_name)?;
    stop_all_running_activities(&mut file_content);
    bartib_file::write_to_file(file_name, &file_content).context(format!("Could not write to file: {}", file_name))
}

// lists all currently runninng activities.
pub fn list_running(file_name: &str) -> Result<()>{
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

    let mut all_projects : Vec<&String> = get_activities(&file_content)
        .map(|activity| &activity.project)
        .collect();

    all_projects.sort_unstable();
    all_projects.dedup();

    for project in all_projects {
        println!("{}", project);
    }

    Ok(())
}

// return last finished activity
pub fn display_last_activity(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let last_activity = get_last_activity(&file_content);

    if let Some(activity) = last_activity {
        output::display_single_activity(&activity);
    } else {
        println!("No activity has been finished yet.")
    }

    Ok(())
}

// continue last activity
pub fn continue_last_activity(file_name: &str, project_name: Option<&str>, activity_description: Option<&str>) -> Result<()> {
    let mut file_content = bartib_file::get_file_content(file_name)?;

    let optional_last_activity = get_last_activity_by_start(&file_content)
        .or(get_last_activity(&file_content));

    if let Some(last_activity) = optional_last_activity {
        let new_activity = activity::Activity::start(
            project_name.unwrap_or(&last_activity.project).to_string(),
            activity_description.unwrap_or(&last_activity.description).to_string()
        );
        stop_all_running_activities(&mut file_content);
        save_new_activity(file_name, &mut file_content, new_activity)
    } else {
        bail!("No activity has been started before.")
    }
}

pub fn start_editor(file_name: &str, optional_editor_command: Option<&str>) -> Result<()> {
    let editor_command = optional_editor_command.context("editor command is missing")?;
    let command = Command::new(editor_command).arg(file_name).spawn();

    match command {
        Ok(mut child) => {
            child.wait().context("editor did not execute")?;
            Ok(())
        }
        Err(e) => {
            Err(anyhow!(e))
        }
    }
}

fn get_last_activity(file_content: &Vec<Line>) -> Option<&Activity> {
    get_activities(&file_content)
        .filter(|activity| activity.is_stopped())
        .max_by_key(|activity| activity.end.unwrap_or(naive::MIN_DATE.and_hms(0, 0, 0)))
}

fn get_last_activity_by_start(file_content: &Vec<Line>) -> Option<&Activity> {
    get_activities(&file_content).max_by_key(|activity| activity.start)
}

fn get_index_of_first_element(length: usize, sub: Option<usize>) -> usize {
    if let Some(s) = sub {
        length.saturating_sub(s)
    } else {
        0
    }
}

fn stop_all_running_activities(file_content: &mut [bartib_file::Line]) {
    for line in file_content {
        if let Ok(activity) = &mut line.activity {
            if !activity.is_stopped() {
                activity.stop();
                println!(
                    "Stopped activity: \"{}\" ({}) started at {} ({})",
                    activity.description,
                    activity.project,
                    activity.start.format(conf::FORMAT_DATETIME),
                    format_util::format_duration(&activity.get_duration()),
                );

                line.set_changed();
            }
        }
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
