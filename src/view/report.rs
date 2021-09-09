use std::collections::BTreeMap;
use std::fmt;

use crate::data::activity;

struct Report<'a> {
    activities : Vec<&'a activity::Activity>
}

impl<'a> Report<'a> {
    fn new() -> Report<'a> {
        Report { activities : Vec::new() }
    }

    fn add(&mut self, a : &'a activity::Activity) {
        self.activities.push(&a);
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
        let longest_project_line = project_map.keys().map(|p| p.chars().count()).max();
        let longest_activity_line = project_map.values().flatten().map(|a| a.description.chars().count() + 4).max();
        let longest_line = get_max_option(longest_project_line, longest_activity_line);

        for (project, activities) in project_map.iter() {
            write!(f, "{}", project)?;

            for activity in activities.iter() {
                write!(f, "{}", activity)?;
            }
        }

        Ok(())
    }
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

    #[test]
    fn get_project_map() {
        let a1 = activity::Activity::start("p1".to_string(), "d1".to_string(), None);
        let a2 = activity::Activity::start("p1".to_string(), "d2".to_string(), None);
        let a3 = activity::Activity::start("p2".to_string(), "d1".to_string(), None);

        let mut r = Report::new();
        r.add(&a1);
        r.add(&a2);
        r.add(&a3);

        let m = r.get_project_map();

        assert_eq!(m.len(), 2);
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
