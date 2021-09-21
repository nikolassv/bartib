use anyhow::Result;

use crate::data::activity;
use crate::data::bartib_file;
use crate::data::getter;
use crate::view::list;

// lists all currently runninng activities.
pub fn list_running(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let running_activities = getter::get_running_activities(&file_content);
    list::list_running_activities(&running_activities);

    Ok(())
}

// lists tracked activities
//
// the activities will be ordered chronologically.
pub fn list(file_name: &str, filter: getter::ActivityFilter, do_group_activities: bool) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities = getter::get_activities(&file_content);
    let mut filtered_activities: Vec<&activity::Activity> =
        getter::filter_activities(activities, &filter).collect();

    filtered_activities.sort_by_key(|activity| activity.start);

    let first_element  =filtered_activities.len().saturating_sub(filter.number_of_activities.unwrap_or(filtered_activities.len()));

    if do_group_activities {
        list::list_activities_grouped_by_date(
            &filtered_activities[first_element..filtered_activities.len()],
        );
    } else {
        let with_start_dates = !filter.date.is_some();
        list::list_activities(
            &filtered_activities[first_element..filtered_activities.len()],
            with_start_dates,
        );
    }

    Ok(())
}

// lists all projects
pub fn list_projects(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let mut all_projects: Vec<&String> = getter::get_activities(&file_content)
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

    let descriptions_and_projects : Vec<(&String, &String)> = getter::get_descriptions_and_projects(&file_content).into_iter().collect();
    list::list_descriptions_and_projects(&descriptions_and_projects[..]);

    Ok(())
}