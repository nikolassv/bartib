use anyhow::{anyhow, bail, Context, Error, Result};
use chrono::NaiveDateTime;
use std::process::Command;

use crate::activity;
use crate::bartib_file;
use crate::conf;
use crate::format_util;
use crate::list_commands;

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
    bartib_file::write_to_file(file_name, &file_content)
        .context(format!("Could not write to file: {}", file_name))
}

// stops all currently running activities
pub fn stop(file_name: &str, time: Option<NaiveDateTime>) -> Result<()> {
    let mut file_content = bartib_file::get_file_content(file_name)?;
    stop_all_running_activities(&mut file_content, time);
    bartib_file::write_to_file(file_name, &file_content)
        .context(format!("Could not write to file: {}", file_name))
}


// continue last activity
pub fn continue_last_activity(
    file_name: &str,
    project_name: Option<&str>,
    activity_description: Option<&str>,
    time: Option<NaiveDateTime>,
) -> Result<()> {
    let mut file_content = bartib_file::get_file_content(file_name)?;

    let optional_last_activity =
        list_commands::get_last_activity_by_start(&file_content).or(list_commands::get_last_activity_by_end(&file_content));

    if let Some(last_activity) = optional_last_activity {
        let new_activity = activity::Activity::start(
            project_name.unwrap_or(&last_activity.project).to_string(),
            activity_description
                .unwrap_or(&last_activity.description)
                .to_string(),
            time,
        );
        stop_all_running_activities(&mut file_content, time);
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