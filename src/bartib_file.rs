use std::io;
use std::fs::{File,OpenOptions};
use std::str::FromStr;
use std::io::{Write,BufReader,BufRead};

use crate::task;

pub enum RowStatus {
	Unchanged, Changed
}

pub struct Row {
	plaintext: String,
	pub task: Result<task::Task, task::TaskError>,
	status: RowStatus
}

impl Row {
	pub fn new(plaintext: &str) -> Row {
		Row {
			plaintext: plaintext.trim().to_string(),
			task: task::Task::from_str(plaintext),
			status: RowStatus::Unchanged
		}
	}
	
	pub fn for_task(task: task::Task) -> Row {
		Row {
			plaintext: "".to_string(),
			task: Ok(task),
			status: RowStatus::Changed
		}
	}
	
	pub fn set_changed(&mut self) {
		self.status = RowStatus::Changed;
	}
}

pub fn get_file_content(file_name: &str) -> Vec<Row> {
	let file_handler = File::open(file_name).unwrap();
	let reader = BufReader::new(file_handler);
	
	reader.lines()
		.filter_map(|line_result| line_result.ok())
		.map(|line| Row::new(&line))
		.collect()
}

pub fn write_to_file(file_name: &str, file_content: &[Row]) -> Result<(), io::Error> {
	let file_handler = get_bartib_file_writable(file_name)?;
	
	for row in file_content {
		match row.status {
			RowStatus::Unchanged => writeln!(&file_handler, "{}", row.plaintext)?,
			RowStatus::Changed => write!(&file_handler, "{}", row.task.as_ref().unwrap())?
		}
	}
	
	Ok(())
}


pub fn get_bartib_file_writable(file_name: &str) -> Result<File, io::Error> {
	OpenOptions::new()
		.create(true)
		.write(true)
		.open(file_name)
}