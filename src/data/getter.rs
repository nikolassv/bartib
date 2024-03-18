use chrono::NaiveDate;
use std::collections::HashSet;
use wildmatch::WildMatch;

use crate::data::activity;
use crate::data::activity::Activity;
use crate::data::bartib_file;
use crate::data::filter::Filters;

pub struct ActivityFilter<'a> {
    pub number_of_activities: Option<usize>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub date: Option<NaiveDate>,
    pub project: Option<&'a str>,
}

#[must_use]
pub fn get_descriptions_and_projects(
    file_content: &[bartib_file::Line],
) -> Vec<(&String, &String)> {
    let mut activities: Vec<&activity::Activity> = get_activities(file_content).collect();
    get_descriptions_and_projects_from_activities(&mut activities)
}

fn get_descriptions_and_projects_from_activities<'a>(
    activities: &mut [&'a Activity],
) -> Vec<(&'a String, &'a String)> {
    activities.sort_by_key(|activity| activity.start);

    /* each activity should be placed in the list in the descending order of when it had been
       started last. To achieve this we reverse the order of the activities before we extract the
       set of descriptions and activities. Afterwards we also reverse the list of descriptions and
       activities.

       e.g. if tasks have been started in this order: a, b, c, a, c the list of descriptions and
           activities should have this order: b, a, c
    */
    activities.reverse();

    let mut known_descriptions_and_projects: HashSet<(&String, &String)> = HashSet::new();
    let mut descriptions_and_projects: Vec<(&String, &String)> = Vec::new();

    for description_and_project in activities.iter().map(|a| (&a.description, &a.project)) {
        if !known_descriptions_and_projects.contains(&description_and_project) {
            known_descriptions_and_projects.insert(description_and_project);
            descriptions_and_projects.push(description_and_project);
        }
    }

    descriptions_and_projects.reverse();
    descriptions_and_projects
}

#[must_use]
pub fn get_running_activities(file_content: &[bartib_file::Line]) -> Vec<&activity::Activity> {
    get_activities(file_content)
        .filter(Filters::active)
        .collect()
}

pub fn get_activities(
    file_content: &[bartib_file::Line],
) -> impl Iterator<Item = &activity::Activity> {
    file_content
        .iter()
        .filter_map(|line: &bartib_file::Line| match &line.activity {
            Ok(activity) => Some(activity),
            Err(_) => {
                println!(
                    "Warning: Ignoring line {}. Please see `bartib check` for further information",
                    line.line_number.unwrap_or(0),
                );
                None
            }
        })
}

pub fn filter_activities<'a>(
    activities: Vec<&'a activity::Activity>,
    filter: &'a ActivityFilter,
) -> Vec<&'a activity::Activity> {
    let from_date: NaiveDate;
    let to_date: NaiveDate;

    if let Some(date) = filter.date {
        from_date = date;
        to_date = date;
    } else {
        from_date = filter.from_date.unwrap_or(NaiveDate::MIN);
        to_date = filter.to_date.unwrap_or(NaiveDate::MAX);
    }

    activities
        .into_iter()
        .filter(move |activity| {
            activity.start.date() >= from_date && activity.start.date() <= to_date
        })
        .filter(move |activity| {
            filter
                .project
                .map_or(true, |p| WildMatch::new(p).matches(&activity.project))
        })
        .collect()
}

#[must_use]
pub fn get_last_activity_by_end(file_content: &[bartib_file::Line]) -> Option<&activity::Activity> {
    get_activities(file_content)
        .filter(|activity| activity.is_stopped())
        .max_by_key(|activity| {
            activity
                .end
                .unwrap_or_else(|| NaiveDate::MIN.and_hms_opt(0, 0, 0).unwrap())
        })
}

#[must_use]
pub fn get_last_activity_by_start(
    file_content: &[bartib_file::Line],
) -> Option<&activity::Activity> {
    get_activities(file_content).max_by_key(|activity| activity.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_descriptions_and_projects_test_simple() {
        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);
        let mut activities = vec![&a1, &a2, &a3];

        let descriptions_and_projects =
            get_descriptions_and_projects_from_activities(&mut activities);

        assert_eq!(descriptions_and_projects.len(), 2);
        assert_eq!(
            *descriptions_and_projects.first().unwrap(),
            (&"d1".to_string(), &"p1".to_string())
        );
        assert_eq!(
            *descriptions_and_projects.get(1).unwrap(),
            (&"d1".to_string(), &"p2".to_string())
        );
    }

    #[test]
    fn get_descriptions_and_projects_test_restarted_activity() {
        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);
        let a3 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let mut activities = vec![&a1, &a2, &a3];

        let descriptions_and_projects =
            get_descriptions_and_projects_from_activities(&mut activities);

        assert_eq!(descriptions_and_projects.len(), 2);
        assert_eq!(
            *descriptions_and_projects.first().unwrap(),
            (&"d1".to_string(), &"p2".to_string())
        );
        assert_eq!(
            *descriptions_and_projects.get(1).unwrap(),
            (&"d1".to_string(), &"p1".to_string())
        );
    }
}
