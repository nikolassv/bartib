use anyhow::Result;

use crate::data::activity;
use crate::data::bartib_file;
use crate::data::getter;
use crate::view::format_util::Format;
use crate::view::report;

pub fn show_report(file_name: &str, filter: getter::ActivityFilter, format: Format) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities = getter::get_activities(&file_content);
    let mut filtered_activities: Vec<&activity::Activity> =
        getter::filter_activities(activities, &filter).collect();

    filtered_activities.sort_by_key(|activity| activity.start);

    let first_element = filtered_activities.len().saturating_sub(
        filter
            .number_of_activities
            .unwrap_or(filtered_activities.len()),
    );

    report::show_activities(&filtered_activities[first_element..filtered_activities.len()], format);

    Ok(())
}
