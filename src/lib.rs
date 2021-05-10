use std::io;
use std::io::{Write,BufReader,BufRead};
use std::fs::{File,OpenOptions};
use std::str::FromStr;

mod conf;
mod project;
mod task;
mod table;
mod format_util;

pub fn start(mut bartib_file: File, project_name: &str, task_description: &str) {
	let project = project::Project(project_name.to_string());
	let task = task::Task::start(project, task_description.to_string());
	
	write!(bartib_file, "{}", task).expect("Could not write new task to file");
}

pub fn list_running(bartib_file: File) {
	let running_tasks = get_running_tasks(bartib_file);
	list_running_tasks(running_tasks);
}

pub fn get_bartib_file_writable(file_name: &str) -> Result<File, io::Error> {
	OpenOptions::new()
		.create(true)
		.append(true)
		.open(file_name)
}

pub fn get_bartib_file_readable(file_name: &str) -> Result<File, io::Error> {
	File::open(file_name)
}

fn get_running_tasks(bartib_file: File) -> Vec<task::Task> {
	let reader = BufReader::new(bartib_file);
	
	reader.lines()
		.filter_map(|line_result| line_result.ok())
		.map(|line| task::Task::from_str(&line))
		.filter_map(|task_result| task_result.ok())
		.filter(|task| !task.is_stopped())
		.collect()
}

fn list_running_tasks(running_tasks: Vec<task::Task>) {
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