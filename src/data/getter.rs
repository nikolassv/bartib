use std::collections::{HashSet, VecDeque};
use chrono::{naive, NaiveDate};

use crate::data::activity;
use crate::data::bartib_file;

pub struct ActivityFilter {
    pub number_of_activities: Option<usize>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub date: Option<NaiveDate>,
}

pub fn get_descriptions_and_projects(file_content: &[bartib_file::Line]) -> VecDeque<(&String, &String)> {
    let mut known_descriptions_and_projects : HashSet<(&String, &String)> = HashSet::new();
    let mut descriptions_and_projects : VecDeque<(&String, &String)> = VecDeque::new();

    // TODO: sortierung nach Start-Zeit
    for description_and_project in get_activities(file_content).map(|a| (&a.description, &a.project))  {
        if !known_descriptions_and_projects.contains(&description_and_project) {
            known_descriptions_and_projects.insert(description_and_project);
            descriptions_and_projects.push_back(description_and_project);
        }

    }

    descriptions_and_projects
}

pub fn get_running_activities(file_content: &[bartib_file::Line]) -> Vec<&activity::Activity> {
    get_activities(file_content)
        .filter(|activity| !activity.is_stopped())
        .collect()
}

pub fn get_activities(file_content: &[bartib_file::Line]) -> impl Iterator<Item = &activity::Activity> {
    file_content
        .iter()
        .map(|line| line.activity.as_ref())
        .filter_map(|activity_result| activity_result.ok())
}

pub fn filter_activities<'a>(
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

pub fn get_last_activity_by_end(file_content: &[bartib_file::Line]) -> Option<&activity::Activity> {
    get_activities(&file_content)
        .filter(|activity| activity.is_stopped())
        .max_by_key(|activity| activity.end.unwrap_or(naive::MIN_DATE.and_hms(0, 0, 0)))
}

pub fn get_last_activity_by_start(file_content: &Vec<bartib_file::Line>) -> Option<&activity::Activity> {
    get_activities(&file_content).max_by_key(|activity| activity.start)
}
