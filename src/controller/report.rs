use anyhow::Result;

use crate::data::activity;
use crate::data::bartib_file;
use crate::data::getter;
use crate::data::processor;
use crate::view::report;

pub fn show_report(
    file_name: &str,
    filter: getter::ActivityFilter,
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

    report::show_activities(&filtered_activities[first_element..filtered_activities.len()]);

    Ok(())
}
