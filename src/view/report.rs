use std::collections::BTreeMap;
use std::fmt;
use std::ops::Add;
use chrono::Duration;
use nu_ansi_term::Style;

use crate::data::activity;
use crate::view::format_util;

struct Report<'a> {
    activities : &'a[&'a activity::Activity]
}

impl<'a> Report<'a> {
    fn new(activities : &'a[&'a activity::Activity]) -> Report<'a> {
        Report { activities }
    }

    fn get_project_map(&self) -> BTreeMap<&str, Vec<&'a activity::Activity>> {
        let mut project_map : BTreeMap<&str, Vec<&activity::Activity>> = BTreeMap::new();

        self.activities.iter().for_each(|a| {
            project_map.entry(&a.project)
                .or_insert_with(Vec::<&'a activity::Activity>::new)
                .push(a);
        });

        project_map
    }
}

impl<'a> fmt::Display for Report<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let project_map = self.get_project_map();
        let longest_line = get_longest_line(&project_map).unwrap_or(0);
        let mut total_duration = Duration::seconds(0);

        for (project, activities) in project_map.iter() {
            let project_duration = sum_duration(&activities);
            total_duration = total_duration.add(project_duration);

            writeln!(f, "{prefix}{project:.<width$} {duration}{suffix}", 
                prefix = Style::new().bold().prefix(),
                project = project, 
                width = longest_line, 
                duration = format_util::format_duration(&project_duration),
                suffix = Style::new().bold().infix(Style::new())
            )?;

            for activity in activities.iter() {
                writeln!(f, "    {activity:.<width$} {duration}", 
                    activity = activity.description, 
                    width = longest_line - 4, 
                    duration = format_util::format_duration(&activity.get_duration())
                )?;
            }
            writeln!(f, "")?;
        }

        if self.activities.is_empty() {
            writeln!(f, "You have not tracked any activities in the given time range")?;
        } else {
            writeln!(f, "{prefix}{total:.<width$} {duration}{suffix}",
                prefix = Style::new().bold().prefix(),
                total = "Total", 
                width = longest_line, 
                duration = format_util::format_duration(&total_duration),
                suffix = Style::new().bold().infix(Style::new())
            )?;
        }    

        Ok(())
    }
}

pub fn show_activities<'a>(activities : &'a[&'a activity::Activity]) {
    let report = Report::new(activities);
    println!("\n{}", report);
}

fn sum_duration(activities : &[&activity::Activity]) -> Duration {
    let mut duration = Duration::seconds(0);

    for activity in activities {
        duration = duration.add(activity.get_duration());
    }

    duration
}

fn get_longest_line(project_map : &BTreeMap<&str, Vec<&activity::Activity>>) -> Option<usize> {
    let longest_project_line = project_map.keys().map(|p| p.chars().count()).max();
    let longest_activity_line = project_map.values().flatten().map(|a| a.description.chars().count() + 4).max();
    get_max_option(longest_project_line, longest_activity_line)
}

fn get_max_option(o1 : Option<usize>, o2: Option<usize>) -> Option<usize> {
    if let Some(s1) = o1 {
        if let Some(s2) = o2 {
            if s1 > s2 { o1 } else { o2 }
        } else {
            o1
        }
    } else {
        o2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn sum_duration_test() {
        let mut activities : Vec<&activity::Activity> = Vec::new();
        assert_eq!(sum_duration(&activities).num_seconds(), 0);

        let mut a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), Some(NaiveDateTime::parse_from_str("2021-09-01 15:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
        a1.end = Some(NaiveDateTime::parse_from_str("2021-09-01 15:20:00", "%Y-%m-%d %H:%M:%S").unwrap()); // 20 * 60 = 1,200 seconds
        let mut a2 = activity::Activity::start("p1".to_string(), "d2".to_string(), Some(NaiveDateTime::parse_from_str("2021-09-01 15:21:00", "%Y-%m-%d %H:%M:%S").unwrap()));
        a2.end = Some(NaiveDateTime::parse_from_str("2021-09-01 16:21:00", "%Y-%m-%d %H:%M:%S").unwrap()); // 60 * 60 = 3,600 seconds
        let mut a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), Some(NaiveDateTime::parse_from_str("2021-09-01 16:21:00", "%Y-%m-%d %H:%M:%S").unwrap()));
        a3.end = Some(NaiveDateTime::parse_from_str("2021-09-02 16:21:00", "%Y-%m-%d %H:%M:%S").unwrap()); // 24 * 60 * 60 = 86,400 seconds

        activities.push(&a1);
        activities.push(&a2);
        activities.push(&a3);

        assert_eq!(sum_duration(&activities).num_seconds(), 91200);
    }

    #[test]
    fn get_project_map() {
        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p1".to_string(), "d2".to_string(), None);
        let a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);

        let mut activities : Vec<&activity::Activity> = Vec::new();
        activities.push(&a1);
        activities.push(&a2);
        activities.push(&a3);
        let r = Report::new(&activities);

        let m = r.get_project_map();

        assert_eq!(m.len(), 2);
    }

    #[test]
    fn get_longest_line_test() {
        let mut activities : Vec<&activity::Activity> = Vec::new();
        let mut r = Report::new(&activities);
        let project_map1 = r.get_project_map();

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
        r = Report::new(&activities);

        // längste Zeile ist Description + 4
        let project_map2 = r.get_project_map();
        assert_eq!(get_longest_line(&project_map2).unwrap(), 6);

        // längste Zeile ist Projektname mit 8 Zeichen
        let a6 = activity::Activity::start("p1234567".to_string(), "d1".to_string(), None);
        activities.push(&a6);
        r = Report::new(&activities);
        let project_map3 = r.get_project_map();
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
