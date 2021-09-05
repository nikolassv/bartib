use std::collections::BTreeMap;

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
}
