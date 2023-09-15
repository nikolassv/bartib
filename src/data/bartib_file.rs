use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

use crate::data::activity;

#[derive(Debug)]
pub enum LineStatus {
    Unchanged,
    Changed,
}

// a line in a bartib file
#[derive(Debug)]
pub struct Line {
    // the plaintext of the line as it has been read from the file
    // we save this to be able write untouched lines back to file without changing them
    pub plaintext: Option<String>,
    // the line number
    pub line_number: Option<usize>,
    // the result of parsing this line to a activity
    pub activity: Result<activity::Activity, activity::ActivityError>,
    // the status of this activity
    status: LineStatus,
}

impl Line {
    // creates a new line struct from plaintext
    #[must_use]
    pub fn new(plaintext: &str, line_number: usize) -> Self {
        Self {
            plaintext: Some(plaintext.trim().to_string()),
            line_number: Some(line_number),
            activity: activity::Activity::from_str(plaintext),
            status: LineStatus::Unchanged,
        }
    }

    // creates a new line from an existing activity
    #[must_use]
    pub fn for_activity(activity: activity::Activity) -> Self {
        Self {
            plaintext: None,
            line_number: None,
            activity: Ok(activity),
            status: LineStatus::Changed,
        }
    }

    // sets the status of the line to changed
    pub fn set_changed(&mut self) {
        self.status = LineStatus::Changed;
    }
}

// reads the content of a file to a vector of lines
pub fn get_file_content(file_name: &str) -> Result<Vec<Line>> {
    let file_handler =
        File::open(file_name).context(format!("Could not read from file: {file_name}"))?;
    let reader = BufReader::new(file_handler);

    let lines = reader
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .map(|(line_number, line)| Line::new(&line, line_number.saturating_add(1)))
        .collect();

    Ok(lines)
}

// writes a vector of lines into a file
pub fn write_to_file(file_name: &str, file_content: &[Line]) -> Result<(), io::Error> {
    let file_handler = get_bartib_file_writable(file_name)?;

    for line in file_content {
        match &line.status {
            LineStatus::Unchanged => {
                if let Some(plaintext) = &line.plaintext {
                    writeln!(&file_handler, "{plaintext}")?
                } else {
                    write!(&file_handler, "{}", line.activity.as_ref().unwrap())?
                }
            }
            LineStatus::Changed => write!(&file_handler, "{}", line.activity.as_ref().unwrap())?,
        }
    }

    Ok(())
}

// create a write handle to a file
fn get_bartib_file_writable(file_name: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)
}
