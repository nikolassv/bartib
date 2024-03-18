use anyhow::Result;
use chrono::Local;

use crate::data::activity;
use crate::data::activity::Activity;
use crate::data::bartib_file;
use crate::data::filter::Filters;
use crate::data::getter;
use crate::data::processor;
use crate::data::processor::StatusReportData;

pub fn show_status(
    file_name: &str,
    filter: getter::ActivityFilter,
    processors: processor::ProcessorList,
    writer: &dyn processor::StatusReportWriter,
) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities: Vec<&Activity> = getter::get_activities(&file_content).collect();

    let processed_activities_bind: Vec<activity::Activity> =
        processor::process_activities(activities, processors);
    let processed_activities: Vec<&activity::Activity> = processed_activities_bind.iter().collect();

    let mut filtered_activities: Vec<&activity::Activity> =
        getter::filter_activities(processed_activities, &filter);

    filtered_activities.sort_by_key(|activity| activity.start);

    let now = Local::now().naive_local();

    let current: Option<&Activity> = filtered_activities
        .clone()
        .into_iter()
        .filter(Filters::active)
        .take(1)
        .last();

    let today = filtered_activities
        .clone()
        .into_iter()
        .filter(Filters::today(now.date()))
        .map(|f| f.get_duration())
        .sum();

    let current_week = filtered_activities
        .clone()
        .into_iter()
        .filter(Filters::current_week(now.date()))
        .map(|f| f.get_duration())
        .sum();

    let current_month = filtered_activities
        .into_iter()
        .filter(Filters::current_month(now.date()))
        .map(|f| f.get_duration())
        .sum();

    let status_report_data = StatusReportData {
        activity: current,
        today,
        current_week,
        current_month,
        project: filter.project,
    };
    writer.process(&status_report_data)
}
