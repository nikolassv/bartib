use chrono::Duration;

use crate::data::activity;
use crate::data::round_util::round_datetime;

pub type ProcessorList = Vec<Box<dyn ActivityProcessor>>;

pub trait ActivityProcessor {
    fn process(&self, activity: &activity::Activity) -> activity::Activity;
}

pub struct RoundProcessor {
    pub round: Duration,
}

impl ActivityProcessor for RoundProcessor {
    fn process(&self, activity: &activity::Activity) -> activity::Activity {
        let start = round_datetime(&activity.start, &self.round);
        let end = activity.end.map(|end| round_datetime(&end, &self.round));

        activity::Activity {
            start,
            end,
            project: activity.project.clone(),
            description: activity.description.clone(),
        }
    }
}

pub fn process_activities(
    activities: Vec<&activity::Activity>,
    processors: ProcessorList,
) -> Vec<activity::Activity> {
    activities
        .into_iter()
        .cloned()
        .map(|activity| {
            processors
                .iter()
                .fold(activity, |activity, processor| processor.process(&activity))
        })
        .collect()
}
