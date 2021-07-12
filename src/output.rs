use std::collections::BTreeMap;
use chrono::NaiveDate;

use crate::task;
use crate::format_util;
use crate::table;
use crate::conf;

// displays a table with tasks
pub fn list_tasks(tasks: &[&task::Task], with_start_dates: bool) {
	if tasks.is_empty() {
		println!("No task to display");
		return
	}

	let mut task_table = table::Table::new(vec!["   ", "Started", "Stopped", "Description", "Project", "Duration"]);

	tasks.iter()
		.map(|t| get_task_table_row(&t, with_start_dates))
		.for_each(|row| task_table.add_row(row));

	println!("\n{}", task_table);
}

// list tasks grouped by the dates of their start time
pub fn list_tasks_grouped_by_date(tasks: &[&task::Task]) {
	if tasks.is_empty() {
		println!("No task to display");
		return
	}

	let tasks_by_date = group_tasks_by_date(tasks);

	for (date, task_list) in tasks_by_date {
		println!("{}", date);
		list_tasks(&task_list, false);
		println!();
	}
}

// displays a table with running tasks (no end time)
pub fn list_running_tasks(running_tasks: &[&task::Task]) {
	if running_tasks.is_empty() {
		println!("No Task is currently running");
	} else {		
		let mut task_table = table::Table::new(vec!["Started At", "Description", "Project", "Duration"]);
		
		running_tasks.iter()
			.map(|task| table::Row::new(vec![
				task.start.format(conf::FORMAT_DATETIME).to_string(),
				task.description.clone(),
				task.project.to_string(),
				format_util::format_duration(&task.get_duration())
			]))
			.for_each(|row| task_table.add_row(row));
			
		println!("\n{}", task_table);
	}
}

// create a row for a task
//
// the date of the end is shown when it is not the same date as the start
fn get_task_table_row(task: &&task::Task, with_start_dates : bool) -> table::Row {
	let display_end = task.end.map_or_else(
		|| "-".to_string(), 
		|end|  if task.start.date() == end.date() {
			end.format(conf::FORMAT_TIME).to_string()
		} else {
			end.format(conf::FORMAT_DATETIME).to_string()

		}
	);

	let start_format = if with_start_dates {conf::FORMAT_DATETIME} else {conf::FORMAT_TIME};

	table::Row::new(vec![
		if !task.is_stopped() {" * ".to_string()} else {" ".to_string()},
		task.start.format(start_format).to_string(),
		display_end,
		task.description.clone(),
		task.project.to_string(),
		format_util::format_duration(&task.get_duration())
	])
}

// groups tasks in vectors of tasks that started at the same day
fn group_tasks_by_date<'a>(tasks: &[&'a task::Task]) -> BTreeMap<NaiveDate, Vec<&'a task::Task>> {
	let mut tasks_by_date = BTreeMap::new();

	for &task in tasks.iter() {
		tasks_by_date.entry(task.start.date()).or_insert(Vec::new()).push(task);
	}

	for task_list in tasks_by_date.values_mut() {
		task_list.sort_by_key(|task| task.start);
	}

	tasks_by_date
}