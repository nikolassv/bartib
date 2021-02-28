use chrono::{DateTime, Local};
use std::fmt;

use crate::project;
use crate::conf;

#[derive(Debug)]
pub struct Task {
	start : DateTime<Local>,
	end : Option<DateTime<Local>>,
	project : project::Project,
	description : String
}

impl Task {
	pub fn start(project : project::Project, description : String) -> Task {
		Task {
			start : Local::now(),
			end: None,
			project,
			description
		}
	}
	
	pub fn stop(&mut self) {
		self.end = Some(Local::now());
	}
}

impl fmt::Display for Task {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {		
		match self.end {
			None => writeln!(f, "{} | {} | {}", self.start.format(conf::FORMAT_DATETIME), self.project, self.description),
			Some(end) => writeln!(f, "{} - {} | {} | {}", self.start.format(conf::FORMAT_DATETIME), end.format(conf::FORMAT_DATETIME), self.project, self.description)
		}
	}
}