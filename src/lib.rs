use std::io;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;

mod conf;
mod project;
mod task;

pub fn start(mut bartib_file: File, project_name: &str, task_description: &str) {
	let project = project::Project(project_name.to_string());
	let task = task::Task::start(project, task_description.to_string());
	
	write!(bartib_file, "{}", task).expect("Could not write new task to file");
}

pub fn get_bartib_file_writable(file_name: &str) -> Result<File, io::Error> {
	OpenOptions::new()
		.create(true)
		.append(true)
		.open(file_name)
}
