use anyhow::Result;
use chrono::NaiveDateTime;
use wildmatch::WildMatch;

use crate::conf;
use crate::data::activity;
use crate::data::activity::Activity;
use crate::data::bartib_file;
use crate::data::getter;
use crate::data::processor;
use crate::view::list;

// lists all currently running activities.
pub fn list_running(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let running_activities = getter::get_running_activities(&file_content);

    list::list_running_activities(&running_activities);

    Ok(())
}

// lists tracked activities
//
// the activities will be ordered chronologically.
pub fn list(
    file_name: &str,
    filter: getter::ActivityFilter,
    do_group_activities: bool,
    processors: processor::ProcessorList,
) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities = getter::get_activities(&file_content).collect();
    let processed_activities_bind: Vec<activity::Activity> =
        processor::process_activities(activities, processors);
    let processed_activities: Vec<&activity::Activity> = processed_activities_bind.iter().collect();

    let mut filtered_activities: Vec<&activity::Activity> =
        getter::filter_activities(processed_activities, &filter);

    filtered_activities.sort_by_key(|activity| activity.start);

    let first_element = filtered_activities.len().saturating_sub(
        filter
            .number_of_activities
            .unwrap_or(filtered_activities.len()),
    );

    if do_group_activities {
        list::list_activities_grouped_by_date(&filtered_activities[first_element..]);
    } else {
        let with_start_dates = filter.date.is_none();
        list::list_activities(&filtered_activities[first_element..], with_start_dates);
    }

    Ok(())
}

// checks the file content for sanity
pub fn sanity_check(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let mut lines_with_activities: Vec<(Option<usize>, Activity)> = file_content
        .into_iter()
        .filter_map(|line| match line.activity {
            Ok(a) => Some((line.line_number, a)),
            Err(_) => None,
        })
        .collect();
    lines_with_activities.sort_unstable_by_key(|(_, activity)| activity.start);

    let mut has_finding: bool = false;
    let mut last_end: Option<NaiveDateTime> = None;

    for (line_number, activity) in lines_with_activities {
        has_finding = !check_sanity(last_end, &activity, line_number) || has_finding;

        if let Some(e) = last_end {
            if let Some(this_end) = activity.end {
                if this_end > e {
                    last_end = Some(this_end);
                }
            }
        } else {
            last_end = activity.end;
        }
    }

    if !has_finding {
        println!("No unusual activities.");
    }

    Ok(())
}

fn check_sanity(
    last_end: Option<NaiveDateTime>,
    activity: &Activity,
    line_number: Option<usize>,
) -> bool {
    let mut sane = true;
    if activity.get_duration().num_milliseconds() < 0 {
        println!("Activity has negative duration");
        sane = false;
    }

    if let Some(e) = last_end {
        if e > activity.start {
            println!("Activity started before another activity ended");
            sane = false;
        }
    }

    if !sane {
        print_activity_with_line(activity, line_number.unwrap_or(0));
    }

    sane
}

fn print_activity_with_line(activity: &Activity, line_number: usize) {
    println!(
        "{} (Started: {}, Ended: {}, Line: {})\n",
        activity.description,
        activity.start.format(conf::FORMAT_DATETIME),
        activity.end.map_or_else(
            || String::from("--"),
            |end| end.format(conf::FORMAT_DATETIME).to_string()
        ),
        line_number
    )
}

// prints all errors that occurred when reading the bartib file
pub fn check(file_name: &str) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let number_of_errors = file_content
        .iter()
        .filter(|line| line.activity.is_err())
        .count();

    if number_of_errors == 0 {
        println!("All lines in the file have been successfully parsed as activities.");
        return Ok(());
    }

    println!("Found {number_of_errors} line(s) with parsing errors");

    file_content
        .iter()
        .filter(|line| line.activity.is_err() && line.plaintext.is_some())
        .for_each(|line| {
            if let Err(e) = &line.activity {
                println!(
                    "\n{}\n  -> {} (Line: {})",
                    line.plaintext.as_ref().unwrap(),
                    e,
                    line.line_number.unwrap_or(0)
                );
            }
        });

    Ok(())
}

// lists all projects
pub fn list_projects(file_name: &str, current: bool, no_quotes: bool) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let mut all_projects: Vec<&String> = getter::get_activities(&file_content)
        .filter(|activity| !(current && activity.is_stopped()))
        .map(|activity| &activity.project)
        .collect();

    all_projects.sort_unstable();
    all_projects.dedup();

    for project in all_projects {
        if no_quotes {
            println!("{project}");
        } else {
            println!("\"{project}\"");
        }
    }

    Ok(())
}

// return last finished activity
pub fn list_last_activities(file_name: &str, number: usize) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;

    let descriptions_and_projects: Vec<(&String, &String)> =
        getter::get_descriptions_and_projects(&file_content);
    let first_element = descriptions_and_projects.len().saturating_sub(number);

    list::list_descriptions_and_projects(&descriptions_and_projects[first_element..]);

    Ok(())
}

// searches for the term in descriptions and projects
pub fn search(file_name: &str, search_term: Option<&str>) -> Result<()> {
    let search_term = search_term
        .map(|term| format!("*{}*", term.to_lowercase()))
        .unwrap_or("".to_string());
    let file_content = bartib_file::get_file_content(file_name)?;

    let descriptions_and_projects: Vec<(&String, &String)> =
        getter::get_descriptions_and_projects(&file_content);
    let search_term_wildmatch = WildMatch::new(&search_term);
    let matches: Vec<(usize, &(&String, &String))> = descriptions_and_projects
        .iter()
        .rev()
        .enumerate()
        .rev()
        .filter(|(_index, (desc, proj))| {
            search_term_wildmatch.matches(&desc.to_lowercase())
                || search_term_wildmatch.matches(&proj.to_lowercase())
        })
        .collect();

    list::list_descriptions_and_projects_with_index(&matches, "No matching activities found");

    Ok(())
}
