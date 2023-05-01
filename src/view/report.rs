use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;

use chrono::Duration;
use nu_ansi_term::Style;
use serde::{Serializer, Serialize};
use serde::ser::{SerializeMap};
use textwrap;

extern crate serde;
extern crate serde_json;

use crate::conf;
use crate::data::activity;
use crate::view::format_util;
use crate::view::format_util::Format;

use super::list::SerializableDuration;

type ProjectMap<'a> = BTreeMap<&'a str, (Vec<&'a activity::Activity>, Duration)>;

struct Report<'a> {
    project_map: ProjectMap<'a>,
    total_duration: Duration,
}

impl<'a> Report<'a> {
    fn new(activities: &'a [&'a activity::Activity]) -> Report<'a> {
        Report {
            project_map: create_project_map(activities),
            total_duration: sum_duration(activities),
        }
    }
}

#[derive(Serialize)]
struct Project<'a> {
    project: &'a str,
    activities: Vec<&'a activity::Activity>,
    duration: SerializableDuration,
}

impl<'a> serde::Serialize for Report<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut report_map = serializer.serialize_map(Some(2))?;
        report_map.serialize_entry(
            "totalDuration",
            &SerializableDuration::from(&self.total_duration),
        )?;
        
        report_map.serialize_entry(
            "projects", 
            &self.project_map.clone().into_iter()
            .map(|(name, (activities, duration))| {
                Project {
                    project: name,
                    activities: activities.clone(),
                    duration: SerializableDuration::from(&duration)
                }
            }).collect::<Vec<_>>()
        )?;
        
        report_map.end()
    }
}

impl<'a> fmt::Display for Report<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut longest_line = get_longest_line(&self.project_map).unwrap_or(0);
        let longest_duration_string = get_longest_duration_string(self).unwrap_or(0);

        let terminal_width = term_size::dimensions_stdout()
            .map(|d| d.0)
            .unwrap_or(conf::DEFAULT_WIDTH);

        if terminal_width < longest_line + longest_duration_string + 1 {
            longest_line = terminal_width - longest_duration_string - 1;
        }

        for (project, (activities, duration)) in self.project_map.iter() {
            print_project_heading(f, project, duration, longest_line, longest_duration_string)?;

            print_descriptions_with_durations(
                f,
                activities,
                longest_line,
                longest_duration_string,
            )?;
            writeln!(f)?;
        }

        print_total_duration(f, self.total_duration, longest_line)?;

        Ok(())
    }
}

pub fn show_activities<'a>(activities: &'a [&'a activity::Activity], format: Format) {
    let report = Report::new(activities);
    match format {
        Format::SHELL => println!("{}", report),
        Format::JSON => println!("{}", serde_json::to_string(&report).unwrap()),
    }
}

fn create_project_map<'a>(activities: &'a [&'a activity::Activity]) -> ProjectMap {
    let mut project_map: ProjectMap = BTreeMap::new();

    activities.iter().for_each(|a| {
        project_map
            .entry(&a.project)
            .or_insert_with(|| (Vec::<&'a activity::Activity>::new(), Duration::seconds(0)))
            .0
            .push(a);
    });

    for (_project, (activities, duration)) in project_map.iter_mut() {
        *duration = sum_duration(activities);
    }

    project_map
}

fn sum_duration(activities: &[&activity::Activity]) -> Duration {
    let mut duration = Duration::seconds(0);

    for activity in activities {
        duration = duration.add(activity.get_duration());
    }

    duration
}

fn print_project_heading(
    f: &mut Formatter,
    project: &&str,
    duration: &Duration,
    longest_line: usize,
    duration_width: usize,
) -> fmt::Result {
    write!(f, "{}", Style::new().bold().prefix())?;
    let project_lines = textwrap::wrap(project, textwrap::Options::new(longest_line));

    for (i, line) in project_lines.iter().enumerate() {
        if i + 1 < project_lines.len() {
            writeln!(f, "{}", line)?;
        } else {
            write!(
                f,
                "{line:.<width$} {duration:>duration_width$}",
                line = line,
                width = longest_line,
                duration = format_util::format_duration(duration),
                duration_width = duration_width
            )?;
        }
    }

    writeln!(f, "{}", Style::new().bold().infix(Style::new()))
}

fn print_descriptions_with_durations<'a>(
    f: &mut fmt::Formatter<'_>,
    activities: &'a [&'a activity::Activity],
    line_width: usize,
    duration_width: usize,
) -> fmt::Result {
    let description_map = group_activities_by_description(activities);
    let indent_string = " ".repeat(conf::REPORT_INDENTATION);
    let wrapping_options = textwrap::Options::new(line_width)
        .initial_indent(&indent_string)
        .subsequent_indent(&indent_string);

    for (description, activities) in description_map.iter() {
        let description_duration = sum_duration(activities);
        let description_lines = textwrap::wrap(description, &wrapping_options);

        for (i, line) in description_lines.iter().enumerate() {
            if i + 1 < description_lines.len() {
                writeln!(f, "{}", line)?;
            } else {
                writeln!(
                    f,
                    "{line:.<width$} {duration:>duration_width$}",
                    line = line,
                    width = line_width,
                    duration = format_util::format_duration(&description_duration),
                    duration_width = duration_width
                )?;
            }
        }
    }

    Ok(())
}

fn print_total_duration(
    f: &mut fmt::Formatter<'_>,
    total_duration: Duration,
    line_width: usize,
) -> fmt::Result {
    writeln!(
        f,
        "{prefix}{total:.<width$} {duration}{suffix}",
        prefix = Style::new().bold().prefix(),
        total = "Total",
        width = line_width,
        duration = format_util::format_duration(&total_duration),
        suffix = Style::new().bold().infix(Style::new())
    )?;

    Ok(())
}

fn group_activities_by_description<'a>(
    activities: &'a [&'a activity::Activity],
) -> BTreeMap<&str, Vec<&'a activity::Activity>> {
    let mut activity_map: BTreeMap<&str, Vec<&'a activity::Activity>> = BTreeMap::new();

    activities.iter().for_each(|a| {
        activity_map
            .entry(&a.description)
            .or_insert_with(Vec::<&'a activity::Activity>::new)
            .push(a);
    });

    activity_map
}

fn get_longest_line(project_map: &ProjectMap) -> Option<usize> {
    let longest_project_line = project_map.keys().map(|p| p.chars().count()).max();
    let longest_activity_line = project_map
        .values()
        .flat_map(|(a, _d)| a)
        .map(|a| a.description.chars().count() + conf::REPORT_INDENTATION)
        .max();
    get_max_option(longest_project_line, longest_activity_line)
}

fn get_longest_duration_string(report: &Report) -> Option<usize> {
    let longest_project_duration = report
        .project_map
        .values()
        .map(|(_a, d)| format_util::format_duration(d))
        .map(|s| s.chars().count())
        .max();
    let longest_activity_duration = report
        .project_map
        .values()
        .flat_map(|(a, _d)| a)
        .map(|a| format_util::format_duration(&a.get_duration()))
        .map(|s| s.chars().count())
        .max();

    let longest_single_duration =
        get_max_option(longest_project_duration, longest_activity_duration);
    let length_of_total_duration = format_util::format_duration(&report.total_duration)
        .chars()
        .count();

    get_max_option(longest_single_duration, Some(length_of_total_duration))
}

fn get_max_option(o1: Option<usize>, o2: Option<usize>) -> Option<usize> {
    if let Some(s1) = o1 {
        if let Some(s2) = o2 {
            if s1 > s2 {
                o1
            } else {
                o2
            }
        } else {
            o1
        }
    } else {
        o2
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use super::*;

    #[test]
    fn sum_duration_test() {
        let mut activities: Vec<&activity::Activity> = Vec::new();
        assert_eq!(sum_duration(&activities).num_seconds(), 0);

        let mut a1 = activity::Activity::start(
            "p1".to_string(),
            "d1".to_string(),
            Some(
                NaiveDateTime::parse_from_str("2021-09-01 15:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        );
        a1.end = Some(
            NaiveDateTime::parse_from_str("2021-09-01 15:20:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ); // 20 * 60 = 1,200 seconds
        let mut a2 = activity::Activity::start(
            "p1".to_string(),
            "d2".to_string(),
            Some(
                NaiveDateTime::parse_from_str("2021-09-01 15:21:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        );
        a2.end = Some(
            NaiveDateTime::parse_from_str("2021-09-01 16:21:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ); // 60 * 60 = 3,600 seconds
        let mut a3 = activity::Activity::start(
            "p2".to_string(),
            "d1".to_string(),
            Some(
                NaiveDateTime::parse_from_str("2021-09-01 16:21:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        );
        a3.end = Some(
            NaiveDateTime::parse_from_str("2021-09-02 16:21:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ); // 24 * 60 * 60 = 86,400 seconds

        activities.push(&a1);
        activities.push(&a2);
        activities.push(&a3);

        assert_eq!(sum_duration(&activities).num_seconds(), 91200);
    }

    #[test]
    fn group_activities_by_project_test() {
        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p1".to_string(), "d2".to_string(), None);
        let a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);

        let activities = vec![&a1, &a2, &a3];
        let m = create_project_map(&activities);

        assert_eq!(m.len(), 2);
        assert_eq!(m.get("p1").unwrap().0.len(), 2);
        assert_eq!(m.get("p2").unwrap().0.len(), 1);
    }

    #[test]
    fn group_activities_by_description_test() {
        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p1".to_string(), "d2".to_string(), None);
        let a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);
        let a4 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);

        let activities = vec![&a1, &a2, &a3, &a4];
        let m = group_activities_by_description(&activities);

        assert_eq!(m.len(), 2);
        assert_eq!(m.get("d1").unwrap().len(), 3);
        assert_eq!(m.get("d2").unwrap().len(), 1);
    }

    #[test]
    fn get_longest_line_test() {
        let mut activities: Vec<&activity::Activity> = Vec::new();
        let project_map1 = create_project_map(&activities);

        // keine Einträge -> keine Längste Zeile
        assert_eq!(get_longest_line(&project_map1), None);

        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p1".to_string(), "d2".to_string(), None);
        let a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);
        let a4 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);
        let a5 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);

        activities.push(&a1);
        activities.push(&a2);
        activities.push(&a3);
        activities.push(&a4);
        activities.push(&a5);

        // längste Zeile ist Description + 4
        let project_map2 = create_project_map(&activities);
        assert_eq!(get_longest_line(&project_map2).unwrap(), 6);

        // längste Zeile ist Projektname mit 8 Zeichen
        let a6 = activity::Activity::start("p1234567".to_string(), "d1".to_string(), None);
        activities.push(&a6);
        let project_map3 = create_project_map(&activities);
        assert_eq!(get_longest_line(&project_map3).unwrap(), 8);
    }

    #[test]
    fn get_max_option_test() {
        assert_eq!(get_max_option(None, None), None);
        assert_eq!(get_max_option(Some(1), None).unwrap(), 1);
        assert_eq!(get_max_option(None, Some(1)).unwrap(), 1);
        assert_eq!(get_max_option(Some(1), Some(1)).unwrap(), 1);
        assert_eq!(get_max_option(Some(2), Some(1)).unwrap(), 2);
        assert_eq!(get_max_option(Some(1), Some(2)).unwrap(), 2);
    }
}
