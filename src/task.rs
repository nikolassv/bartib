use chrono::{NaiveDateTime, Local};
use std::fmt;
use std::str::{FromStr, Chars};
use thiserror::Error;

use crate::project;
use crate::conf;

#[derive(Debug)]
pub struct Task {
	start : NaiveDateTime,
	end : Option<NaiveDateTime>,
	
	project : project::Project,
	description : String
}

#[derive(Error,Debug)]
pub enum TaskError {
	#[error("could not parse date or time of task")]
	DateTimeParseError,
	#[error("could not parse task")]
	GeneralParseError
}

impl Task {
	pub fn start(project : project::Project, description : String) -> Task {
		Task {
			start : Local::now().naive_local(),
			end: None,
			project,
			description
		}
	}
	
	pub fn stop(&mut self) {
		self.end = Some(Local::now().naive_local());
	}
	
	pub fn is_stopped(&self) -> bool {
		self.end.is_some()
	}
}

impl fmt::Display for Task {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let escaped_project_name = escape_special_chars(&format!("{}", self.project));
		let escaped_description = escape_special_chars(&self.description);
		
		match self.end {
			None => writeln!(f, "{} | {} | {}", self.start.format(conf::FORMAT_DATETIME), escaped_project_name, escaped_description),
			Some(end) => writeln!(f, "{} - {} | {} | {}", self.start.format(conf::FORMAT_DATETIME), end.format(conf::FORMAT_DATETIME), escaped_project_name, escaped_description)
		}
	}
}

// escapes the pipe character, so we can use it to seperate the distinct parts of a task
fn escape_special_chars(s: &str) -> String {
	s.replace("\\", "\\\\").replace("|", "\\|")
}

impl FromStr for Task {
	type Err = TaskError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts : Vec<String> = split_with_escaped_delimeter(s).collect();
				
		if parts.len() < 2 {
			return Err(TaskError::GeneralParseError);
		}
	
		let time_parts : Vec<&str> = parts[0].split(" - ").collect();
		
		let starttime = match NaiveDateTime::parse_from_str(time_parts[0].trim(), conf::FORMAT_DATETIME) {
			Ok(t) => t,
			Err(_) => {
				return Err(TaskError::DateTimeParseError)
			}
		};
		
		let endtime: Option<NaiveDateTime>;
		
		if time_parts.len() > 1 {
			endtime = match NaiveDateTime::parse_from_str(time_parts[1].trim(), conf::FORMAT_DATETIME) {
				Ok(t) => Some(t),
				Err(_) => {
					return Err(TaskError::DateTimeParseError)
				}
			}
		} else {
			endtime = None;
		}
		
		let project = parts[1].trim();
		let description = if parts.len() > 2 { parts[2].trim() } else {	"" };
		
		let task = Task{
			start: starttime,
			end: endtime,
			project: project::Project(project.to_string()),
			description: description.to_string()
		};
		
		Ok(task)
	}
}

/**
 * an iterator for splitting strings at the pipe character "|"
 * 
 * this iterator splits strings at the pipe character. It ignores all pipe characters
 * that are properly escaped by backslash.
 */
struct StringSplitter<'a> {
	chars: Chars<'a>
}

impl Iterator for StringSplitter<'_> {
	type Item = String;
	
	fn next(&mut self) -> Option<String> {
		let mut next_char = self.chars.next();
		
		if next_char == None {
			return None;
		}
		
		let mut escaped = false;
		let mut collector = String::new();
		
		loop {
			match next_char {
				Some('\\') if !escaped => {
					escaped = true;
				},
				Some('|') if !escaped => {
					return Some(collector);
				},
				Some(c) => {
					escaped = false;
					collector.push(c);
				},
				None => return Some(collector)
			}
			
			next_char = self.chars.next();
		}
	}
}

fn split_with_escaped_delimeter(s: &str) -> StringSplitter {
	StringSplitter{chars: s.chars()}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{Datelike, Timelike};
	
	#[test]
	fn start() {
		let t = Task::start(project::Project("test project".to_string()), "test description".to_string());		
		assert_eq!(t.description, "test description".to_string());
		assert_eq!(t.project.0, "test project".to_string());
		assert_eq!(t.end, None);
	}

	#[test]
	fn stop() {
		let mut t = Task::start(project::Project("test project".to_string()), "test description".to_string());		
		t.stop();		
		assert_ne!(t.end, None);
	}


	#[test]
	fn display() {
		let mut t = Task::start(project::Project("test project| 1".to_string()), "test\\description".to_string());		
		t.start = NaiveDateTime::parse_from_str("2021-02-16 16:14", conf::FORMAT_DATETIME).unwrap();		
		assert_eq!(format!("{}", t), "2021-02-16 16:14 | test project\\| 1 | test\\\\description\n");		
		t.end = Some(NaiveDateTime::parse_from_str("2021-02-16 18:23" , conf::FORMAT_DATETIME).unwrap());		
		assert_eq!(format!("{}", t), "2021-02-16 16:14 - 2021-02-16 18:23 | test project\\| 1 | test\\\\description\n");
	}
	
	#[test]
	fn from_str_running_task() {
		let t = Task::from_str("2021-02-16 16:14 | test project | test description").unwrap();
		
		assert_eq!(t.start.date().year(), 2021);
		assert_eq!(t.start.date().month(), 2);
		assert_eq!(t.start.date().day(), 16);
		
		assert_eq!(t.start.time().hour(), 16);
		assert_eq!(t.start.time().minute(), 14);
		
		assert_eq!(t.description, "test description".to_string());
		assert_eq!(t.project.0, "test project".to_string());
		assert_eq!(t.end, None);		
	}
	
	#[test]
	fn from_str_running_task_no_description() {
		let t = Task::from_str("2021-02-16 16:14 | test project").unwrap();
		
		assert_eq!(t.start.date().year(), 2021);
		assert_eq!(t.start.date().month(), 2);
		assert_eq!(t.start.date().day(), 16);
		
		assert_eq!(t.start.time().hour(), 16);
		assert_eq!(t.start.time().minute(), 14);
		
		assert_eq!(t.description, "".to_string());
		assert_eq!(t.project.0, "test project".to_string());
		assert_eq!(t.end, None);		
	}
	
	#[test]
	fn from_str_stopped_task() {
		let t = Task::from_str("2021-02-16 16:14 - 2021-02-16 18:23 | test project | test description").unwrap();
		
		assert_ne!(t.end, None);
		
		let end = t.end.unwrap();
		
		assert_eq!(end.date().year(), 2021);
		assert_eq!(end.date().month(), 2);
		assert_eq!(end.date().day(), 16);
		
		assert_eq!(end.time().hour(), 18);
		assert_eq!(end.time().minute(), 23);

	}

	#[test]
	fn from_str_escaped_chars() {
		let t = Task::from_str("2021-02-16 16:14 - 2021-02-16 18:23 | test project\\| 1 | test\\\\description").unwrap();
		
		assert_eq!(t.project.0, "test project| 1");
		assert_eq!(t.description, "test\\description");
	}
	
	#[test]
	fn string_roundtrip() {
		let mut t = Task::start(project::Project("ex\\ample\\\\pro|ject".to_string()), "e\\\\xam|||ple tas\t\t\nk".to_string());
		t.stop();
		let t2 = Task::from_str(format!("{}", t).as_str()).unwrap();
		
		assert_eq!(t.start.with_second(0).unwrap().with_nanosecond(0).unwrap(), t2.start);
		assert_eq!(t.end.unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap(), t2.end.unwrap());
		
		assert_eq!(t.project.0, t2.project.0);
		assert_eq!(t.description, t2.description);
	}
	
	#[test]
	fn from_str_errors() {
		let t = Task::from_str("2021 test project");
		assert!(matches!(t, Err(TaskError::GeneralParseError)));
		
		let t = Task::from_str("asb - 2021- | project");
		assert!(matches!(t, Err(TaskError::DateTimeParseError)));
	}
}