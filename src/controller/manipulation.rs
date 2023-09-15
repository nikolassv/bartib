use anyhow::{anyhow, bail, Context, Error, Result};
use chrono::NaiveDateTime;
use std::process::Command;

use crate::conf;
use crate::data::activity;
use crate::data::bartib_file;
use crate::data::getter;
use crate::view::format_util;

// starts a new activity
pub fn start(
    file_name: &str,
    project_name: &str,
    activity_description: &str,
    time: Option<NaiveDateTime>,
) -> Result<()> {
    let mut file_content: Vec<bartib_file::Line> = Vec::new();

    if let Ok(mut previous_file_content) = bartib_file::get_file_content(file_name) {
        // if we start a new activities programaticly, we stop all other activities first.
        // However, we must not assume that there is always only one activity
        // running as the user may have started activities manually
        stop_all_running_activities(&mut previous_file_content, time);

        file_content.append(&mut previous_file_content);
    }

    let activity = activity::Activity::start(
        project_name.to_string(),
        activity_description.to_string(),
        time,
    );

    save_new_activity(file_name, &mut file_content, activity)
}

fn save_new_activity(
    file_name: &str,
    file_content: &mut Vec<bartib_file::Line>,
    activity: activity::Activity,
) -> Result<(), Error> {
    println!(
        "Started activity: \"{}\" ({}) at {}",
        activity.description,
        activity.project,
        activity.start.format(conf::FORMAT_DATETIME)
    );

    file_content.push(bartib_file::Line::for_activity(activity));
    bartib_file::write_to_file(file_name, file_content)
        .context(format!("Could not write to file: {file_name}"))
}

pub fn change(
    file_name: &str,
    project_name: Option<&str>,
    activity_description: Option<&str>,
    time: Option<NaiveDateTime>,
) -> Result<()> {
    let mut file_content = bartib_file::get_file_content(file_name)?;

    for line in &mut file_content {
        if let Ok(activity) = &mut line.activity {
            if !activity.is_stopped() {
                let mut changed = false;

                if let Some(project_name) = project_name {
                    activity.project = project_name.to_string();
                    changed = true;
                }

                if let Some(activity_description) = activity_description {
                    activity.description = activity_description.to_string();
                    changed = true;
                }

                if let Some(time) = time {
                    activity.start = time;
                    changed = true;
                }

                if changed {
                    println!(
                        "Changed activity: \"{}\" ({}) started at {}",
                        activity.description,
                        activity.project,
                        activity.start.format(conf::FORMAT_DATETIME)
                    );
                    line.set_changed();
                }
            }
        }
    }
    bartib_file::write_to_file(file_name, &file_content)
        .context(format!("Could not write to file: {file_name}"))
}

// stops all currently running activities
pub fn stop(file_name: &str, time: Option<NaiveDateTime>) -> Result<()> {
    let mut file_content = bartib_file::get_file_content(file_name)?;
    stop_all_running_activities(&mut file_content, time);
    bartib_file::write_to_file(file_name, &file_content)
        .context(format!("Could not write to file: {file_name}"))
}

// cancels all currently running activities
pub fn cancel(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let mut new_file_content: Vec<bartib_file::Line> = Vec::new();

    for line in file_content {
        match &line.activity {
            Ok(activity) => {
                if activity.is_stopped() {
                    new_file_content.push(line);
                } else {
                    println!(
                        "Canceled activity: \"{}\" ({}) started at {}",
                        activity.description,
                        activity.project,
                        activity.start.format(conf::FORMAT_DATETIME)
                    );
                }
            }
            Err(_) => new_file_content.push(line),
        }
    }

    bartib_file::write_to_file(file_name, &new_file_content)
        .context(format!("Could not write to file: {file_name}"))
}

// continue last activity
pub fn continue_last_activity(
    file_name: &str,
    project_name: Option<&str>,
    activity_description: Option<&str>,
    time: Option<NaiveDateTime>,
    number: usize,
) -> Result<()> {
    let mut file_content = bartib_file::get_file_content(file_name)?;

    let descriptions_and_projects: Vec<(&String, &String)> =
        getter::get_descriptions_and_projects(&file_content);

    if descriptions_and_projects.is_empty() {
        bail!("No activity has been started before.")
    }

    if number > descriptions_and_projects.len() {
        bail!(format!(
            "Less than {} distinct activities have been logged yet",
            number
        ));
    }

    let i = descriptions_and_projects
        .len()
        .saturating_sub(number)
        .saturating_sub(1);
    let optional_description_and_project = descriptions_and_projects.get(i);

    if let Some((description, project)) = optional_description_and_project {
        let new_activity = activity::Activity::start(
            project_name.unwrap_or(project).to_string(),
            activity_description.unwrap_or(description).to_string(),
            time,
        );
        stop_all_running_activities(&mut file_content, time);
        save_new_activity(file_name, &mut file_content, new_activity)
    } else {
        bail!(format!(
            "Less than {} distinct activities have been logged yet",
            number
        ));
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
        Err(e) => Err(anyhow!(e)),
    }
}

fn stop_all_running_activities(
    file_content: &mut [bartib_file::Line],
    time: Option<NaiveDateTime>,
) {
    for line in file_content {
        if let Ok(activity) = &mut line.activity {
            if !activity.is_stopped() {
                activity.stop(time);
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
