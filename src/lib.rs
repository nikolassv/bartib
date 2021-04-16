use std::io;
use std::io::{Write,BufReader,BufRead};
use std::fs::{File,OpenOptions};
use std::str::FromStr;

mod conf;
mod project;
mod task;

pub fn start(mut bartib_file: File, project_name: &str, task_description: &str) {
	let project = project::Project(project_name.to_string());
	let task = task::Task::start(project, task_description.to_string());
	
	write!(bartib_file, "{}", task).expect("Could not write new task to file");
}

pub fn list_running(bartib_file: File) {

	let reader = BufReader::new(bartib_file);
	
	reader.lines()
		.filter_map(|line_result| line_result.ok())
		.map(|line| task::Task::from_str(&line))
		.filter_map(|task_result| task_result.ok())
		.filter(|task| !task.is_stopped())
		.for_each(|task| print!("{}", task));
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
