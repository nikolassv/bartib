use chrono::NaiveDate;
use chrono::naive;

pub mod conf;
mod project;
mod activity;
mod table;
mod format_util;
mod output;
pub mod bartib_file;

pub struct ActivityFilter {
	pub number_of_activities: Option<usize>,
	pub from_date: Option<NaiveDate>,
	pub to_date: Option<NaiveDate>,
	pub date: Option<NaiveDate>
}

// starts a new activity
pub fn start(file_name: &str, project_name: &str, activity_description: &str) {
	let mut file_content = bartib_file::get_file_content(file_name);
	
	// if we start a new activities programaticly, we stop all other activities first.
	// However, we must not assume that there is always only one activity
	// running as the user may have started activities manually
	stop_all_running_activities(&mut file_content);
	
	let project = project::Project(project_name.to_string());
	let activity = activity::Activity::start(project, activity_description.to_string());	
	
	println!("Started activity: \"{}\" ({}) at {}", activity_description, project_name, activity.start.format(conf::FORMAT_DATETIME));
	
	file_content.push(bartib_file::Line::for_activity(activity));
	bartib_file::write_to_file(file_name, &file_content).expect("Could not write to file");
}

// stops all currently running activities
pub fn stop(file_name: &str) {
	let mut file_content = bartib_file::get_file_content(file_name);	
	stop_all_running_activities(&mut file_content);	
	bartib_file::write_to_file(file_name, &file_content).expect("Could not write to file");
}

// lists all currently runninng activities.
pub fn list_running(file_name: &str) {
	let file_content = bartib_file::get_file_content(file_name);
	let running_activities = get_running_activities(&file_content);
	output::list_running_activities(&running_activities);
}

// lists tracked activities
//
// the activities will be ordered chronologically. 
pub fn list(file_name: &str, filter: ActivityFilter, do_group_activities: bool) {
	let file_content = bartib_file::get_file_content(file_name);
	let activities = get_activities(&file_content);
	let mut filtered_activities : Vec<&activity::Activity> = filter_activities(activities, &filter).collect();
	
	filtered_activities.sort_by_key(|activity| activity.start);

	let first_element = get_index_of_first_element(filtered_activities.len(), filter.number_of_activities);

	if do_group_activities {
		output::list_activities_grouped_by_date(&filtered_activities[first_element .. filtered_activities.len()]);
	} else {
		let with_start_dates = !filter.date.is_some();
		output::list_activities(&filtered_activities[first_element .. filtered_activities.len()], with_start_dates);
	}
}

fn get_index_of_first_element(length: usize, sub: Option<usize>) -> usize {
	if let Some(s) = sub {
		length.saturating_sub(s)
	} else {
		0
	}
}

fn stop_all_running_activities(file_content: &mut [bartib_file::Line]) {
	for line in file_content {
		if let Ok(activity) = &mut line.activity {
			if !activity.is_stopped() {
				activity.stop();
				line.set_changed();
			}
		}
	}
}

fn get_running_activities(file_content: &[bartib_file::Line]) -> Vec<&activity::Activity> {
	get_activities(file_content)
		.filter(|activity| !activity.is_stopped())
		.collect()
}

fn get_activities(file_content: &[bartib_file::Line]) -> impl Iterator<Item = &activity::Activity> {
	file_content.iter()
		.map(|line| line.activity.as_ref())
		.filter_map(|activity_result| activity_result.ok())
}

fn filter_activities<'a>(activities : impl Iterator<Item = &'a activity::Activity>, filter : &ActivityFilter) -> impl Iterator<Item = &'a activity::Activity> {
	let from_date : NaiveDate;
	let to_date : NaiveDate;

	if let Some(date) = filter.date {
		from_date = date;
		to_date = date;
	} else {
		from_date = filter.from_date.unwrap_or(naive::MIN_DATE);
		to_date = filter.to_date.unwrap_or(naive::MAX_DATE);
	}

	activities.filter(move |activity| activity.start.date() >= from_date && activity.start.date() <= to_date)
}