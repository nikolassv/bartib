#[cfg(not(feature = "second-precision"))]
use chrono::DurationRound;
#[cfg(feature = "second-precision")]
use chrono::Timelike;
use chrono::{Duration, Local, NaiveDateTime};
use std::fmt;
use std::str::{Chars, FromStr};
use thiserror::Error;

use crate::conf;

#[derive(Debug, Clone)]
pub struct Activity {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,

    pub project: String,
    pub description: String,
}

#[derive(Error, Debug)]
pub enum ActivityError {
    #[error("could not parse date or time of activity")]
    DateTimeParseError,
    #[error("could not parse activity")]
    GeneralParseError,
}

impl Activity {
    #[must_use]
    pub fn start(project: String, description: String, time: Option<NaiveDateTime>) -> Self {
        Self {
            start: time.unwrap_or_else(|| Local::now().naive_local()),
            end: None,
            project,
            description,
        }
    }

    pub fn stop(&mut self, time: Option<NaiveDateTime>) {
        self.end = time.or_else(|| Some(Local::now().naive_local()));
    }

    #[must_use]
    pub fn is_stopped(&self) -> bool {
        self.end.is_some()
    }

    #[must_use]
    pub fn get_duration(&self) -> Duration {
        if let Some(end) = self.end {
            end.signed_duration_since(self.start)
        } else {
            Local::now().naive_local().signed_duration_since(self.start)
        }
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let escaped_project_name = escape_special_chars(self.project.as_str());
        let escaped_description = escape_special_chars(&self.description);

        match self.end {
            None => writeln!(
                f,
                "{} | {} | {}",
                self.start.format(conf::FORMAT_DATETIME),
                escaped_project_name,
                escaped_description
            ),
            Some(end) => writeln!(
                f,
                "{} - {} | {} | {}",
                self.start.format(conf::FORMAT_DATETIME),
                end.format(conf::FORMAT_DATETIME),
                escaped_project_name,
                escaped_description
            ),
        }
    }
}

// escapes the pipe character, so we can use it to separate the distinct parts of a activity
fn escape_special_chars(s: &str) -> String {
    s.replace('\\', "\\\\").replace('|', "\\|")
}

impl FromStr for Activity {
    type Err = ActivityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = split_with_escaped_delimiter(s).collect();

        if parts.len() < 2 {
            return Err(ActivityError::GeneralParseError);
        }

        let time_parts: Vec<&str> = parts[0].split(" - ").collect();

        let starttime = parse_timepart(time_parts[0])?;
        let endtime: Option<NaiveDateTime> = if time_parts.len() > 1 {
            Some(parse_timepart(time_parts[1])?)
        } else {
            None
        };

        let project = parts[1].trim();
        let description = if parts.len() > 2 { parts[2].trim() } else { "" };

        let activity = Self {
            start: starttime,
            end: endtime,
            project: project.to_string(),
            description: description.to_string(),
        };

        Ok(activity)
    }
}

#[cfg(not(feature = "second-precision"))]
fn parse_timepart(time_part: &str) -> Result<NaiveDateTime, ActivityError> {
    match NaiveDateTime::parse_from_str(time_part.trim(), conf::FORMAT_DATETIME) {
        Ok(datetime) => Ok(datetime),
        Err(_) => {
            // Presume that the timestamp has second-precision and that is the cause of the error
            match NaiveDateTime::parse_from_str(
                time_part.trim(),
                conf::FORMAT_SECOND_PRECISION_DATETIME,
            ) {
                Ok(datetime) => {
                    // Successfully parsed timestamp. Notify the user about the mismatch
                    // and round to the nearest minute
                    eprintln!("WARNING: Bartib log encountered timestamps with minute precision.");
                    eprintln!("This version of Bartib has been compiled with second precision");

                    let datetime = datetime
                        .duration_round(Duration::minutes(1))
                        .map_err(|_| ActivityError::DateTimeParseError)?;

                    Ok(datetime)
                }
                Err(_) => Err(ActivityError::DateTimeParseError),
            }
        }
    }
}

#[cfg(feature = "second-precision")]
fn parse_timepart(time_part: &str) -> Result<NaiveDateTime, ActivityError> {
    match NaiveDateTime::parse_from_str(time_part.trim(), conf::FORMAT_DATETIME) {
        Ok(datetime) => Ok(datetime),
        Err(_) => {
            // Presume that the timestamp has minute-precision and that is the cause of the error
            match NaiveDateTime::parse_from_str(
                time_part.trim(),
                conf::FORMAT_MINUTE_PRECISION_DATETIME,
            ) {
                Ok(datetime) => {
                    // Successfully parsed timestamp. Notify the user about the mismatch
                    // and set seconds to zero
                    eprintln!("WARNING: Bartib log encountered timestamps with minute precision.");
                    eprintln!("This version of Bartib has been compiled with second precision");

                    let datetime = datetime
                        .with_second(0)
                        .ok_or(ActivityError::DateTimeParseError)?;

                    Ok(datetime)
                }
                Err(_) => Err(ActivityError::DateTimeParseError),
            }
        }
    }
}

/**
 * an iterator for splitting strings at the pipe character "|"
 *
 * this iterator splits strings at the pipe character. It ignores all pipe characters
 * that are properly escaped by backslash.
 */
struct StringSplitter<'a> {
    chars: Chars<'a>,
}

impl Iterator for StringSplitter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut next_char = self.chars.next();

        next_char?;

        let mut escaped = false;
        let mut collector = String::new();

        loop {
            match next_char {
                Some('\\') if !escaped => {
                    escaped = true;
                }
                Some('|') if !escaped => {
                    return Some(collector);
                }
                Some(c) => {
                    escaped = false;
                    collector.push(c);
                }
                None => return Some(collector),
            }

            next_char = self.chars.next();
        }
    }
}

fn split_with_escaped_delimiter(s: &str) -> StringSplitter {
    StringSplitter { chars: s.chars() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};
    use std::option::Option::None;

    #[test]
    fn start() {
        let t = Activity::start(
            "test project".to_string(),
            "test description".to_string(),
            None,
        );
        assert_eq!(t.description, "test description".to_string());
        assert_eq!(t.project, "test project".to_string());
        assert_eq!(t.end, None);
    }

    #[test]
    fn stop() {
        let mut t = Activity::start(
            "test project".to_string(),
            "test description".to_string(),
            None,
        );
        t.stop(None);
        assert_ne!(t.end, None);
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn display() {
        let mut t = Activity::start(
            "test project| 1".to_string(),
            "test\\description".to_string(),
            None,
        );
        t.start = NaiveDateTime::parse_from_str("2021-02-16 16:14", conf::FORMAT_DATETIME).unwrap();
        assert_eq!(
            format!("{t}"),
            "2021-02-16 16:14 | test project\\| 1 | test\\\\description\n"
        );
        t.end =
            Some(NaiveDateTime::parse_from_str("2021-02-16 18:23", conf::FORMAT_DATETIME).unwrap());
        assert_eq!(
            format!("{t}"),
            "2021-02-16 16:14 - 2021-02-16 18:23 | test project\\| 1 | test\\\\description\n"
        );
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn display() {
        let mut t = Activity::start(
            "test project| 1".to_string(),
            "test\\description".to_string(),
            None,
        );
        t.start =
            NaiveDateTime::parse_from_str("2021-02-16 16:14:53", conf::FORMAT_DATETIME).unwrap();
        assert_eq!(
            format!("{t}"),
            "2021-02-16 16:14:53 | test project\\| 1 | test\\\\description\n"
        );
        t.end = Some(
            NaiveDateTime::parse_from_str("2021-02-16 18:23:17", conf::FORMAT_DATETIME).unwrap(),
        );
        assert_eq!(
            format!("{t}"),
            "2021-02-16 16:14:53 - 2021-02-16 18:23:17 | test project\\| 1 | test\\\\description\n"
        );
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn from_str_running_activity() {
        let t = Activity::from_str("2021-02-16 16:14 | test project | test description").unwrap();

        assert_eq!(t.start.date().year(), 2021);
        assert_eq!(t.start.date().month(), 2);
        assert_eq!(t.start.date().day(), 16);

        assert_eq!(t.start.time().hour(), 16);
        assert_eq!(t.start.time().minute(), 14);

        assert_eq!(t.description, "test description".to_string());
        assert_eq!(t.project, "test project".to_string());
        assert_eq!(t.end, None);
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn from_str_running_activity() {
        let t =
            Activity::from_str("2021-02-16 16:14:53 | test project | test description").unwrap();

        assert_eq!(t.start.date().year(), 2021);
        assert_eq!(t.start.date().month(), 2);
        assert_eq!(t.start.date().day(), 16);

        assert_eq!(t.start.time().hour(), 16);
        assert_eq!(t.start.time().minute(), 14);
        assert_eq!(t.start.time().second(), 53);

        assert_eq!(t.description, "test description".to_string());
        assert_eq!(t.project, "test project".to_string());
        assert_eq!(t.end, None);
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn from_str_running_activity_no_description() {
        let t = Activity::from_str("2021-02-16 16:14 | test project").unwrap();

        assert_eq!(t.start.date().year(), 2021);
        assert_eq!(t.start.date().month(), 2);
        assert_eq!(t.start.date().day(), 16);

        assert_eq!(t.start.time().hour(), 16);
        assert_eq!(t.start.time().minute(), 14);

        assert_eq!(t.description, String::new());
        assert_eq!(t.project, "test project".to_string());
        assert_eq!(t.end, None);
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn from_str_running_activity_no_description() {
        let t = Activity::from_str("2021-02-16 16:14:53 | test project").unwrap();

        assert_eq!(t.start.date().year(), 2021);
        assert_eq!(t.start.date().month(), 2);
        assert_eq!(t.start.date().day(), 16);

        assert_eq!(t.start.time().hour(), 16);
        assert_eq!(t.start.time().minute(), 14);
        assert_eq!(t.start.time().second(), 53);

        assert_eq!(t.description, String::new());
        assert_eq!(t.project, "test project".to_string());
        assert_eq!(t.end, None);
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn from_str_stopped_activity() {
        let t = Activity::from_str(
            "2021-02-16 16:14 - 2021-02-16 18:23 | test project | test description",
        )
        .unwrap();

        assert_ne!(t.end, None);

        let end = t.end.unwrap();

        assert_eq!(end.date().year(), 2021);
        assert_eq!(end.date().month(), 2);
        assert_eq!(end.date().day(), 16);

        assert_eq!(end.time().hour(), 18);
        assert_eq!(end.time().minute(), 23);
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn from_str_stopped_activity() {
        let t = Activity::from_str(
            "2021-02-16 16:14:53 - 2021-02-16 18:23:17 | test project | test description",
        )
        .unwrap();

        assert_ne!(t.end, None);

        let end = t.end.unwrap();

        assert_eq!(end.date().year(), 2021);
        assert_eq!(end.date().month(), 2);
        assert_eq!(end.date().day(), 16);

        assert_eq!(end.time().hour(), 18);
        assert_eq!(end.time().minute(), 23);
        assert_eq!(end.time().second(), 17);
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn from_str_escaped_chars() {
        let t = Activity::from_str(
            "2021-02-16 16:14 - 2021-02-16 18:23 | test project\\| 1 | test\\\\description",
        )
        .unwrap();

        assert_eq!(t.project, "test project| 1");
        assert_eq!(t.description, "test\\description");
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn from_str_escaped_chars() {
        let t = Activity::from_str(
            "2021-02-16 16:14:53 - 2021-02-16 18:23:17 | test project\\| 1 | test\\\\description",
        )
        .unwrap();

        assert_eq!(t.project, "test project| 1");
        assert_eq!(t.description, "test\\description");
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn string_roundtrip() {
        let mut t = Activity::start(
            "ex\\ample\\\\pro|ject".to_string(),
            "e\\\\xam|||ple tas\t\t\nk".to_string(),
            None,
        );
        t.stop(None);
        let t2 = Activity::from_str(format!("{t}").as_str()).unwrap();

        assert_eq!(
            t.start.with_second(0).unwrap().with_nanosecond(0).unwrap(),
            t2.start
        );
        assert_eq!(
            t.end
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap(),
            t2.end.unwrap()
        );

        assert_eq!(t.project, t2.project);
        assert_eq!(t.description, t2.description);
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn string_roundtrip() {
        let mut t = Activity::start(
            "ex\\ample\\\\pro|ject".to_string(),
            "e\\\\xam|||ple tas\t\t\nk".to_string(),
            None,
        );
        t.stop(None);
        let t2 = Activity::from_str(format!("{t}").as_str()).unwrap();

        assert_eq!(t.start.with_nanosecond(0).unwrap(), t2.start);
        assert_eq!(t.end.unwrap().with_nanosecond(0).unwrap(), t2.end.unwrap());

        assert_eq!(t.project, t2.project);
        assert_eq!(t.description, t2.description);
    }

    #[test]
    fn from_str_errors() {
        let t = Activity::from_str("2021 test project");
        assert!(matches!(t, Err(ActivityError::GeneralParseError)));

        let t = Activity::from_str("asb - 2021- | project");
        assert!(matches!(t, Err(ActivityError::DateTimeParseError)));
    }

    #[test]
    #[cfg(not(feature = "second-precision"))]
    fn from_str_second_precision_encountered_when_minute_precision_expected() {
        let t = Activity::from_str(
            "2021-02-16 16:14:53 - 2021-02-16 18:23:17 | test project | test description",
        )
        .unwrap();

        assert_eq!(t.start.date().year(), 2021);
        assert_eq!(t.start.date().month(), 2);
        assert_eq!(t.start.date().day(), 16);

        assert_eq!(t.start.time().hour(), 16);
        assert_eq!(t.start.time().minute(), 15);
        assert_eq!(t.start.time().second(), 0);

        assert_ne!(t.end, None);

        let end = t.end.unwrap();

        assert_eq!(end.date().year(), 2021);
        assert_eq!(end.date().month(), 2);
        assert_eq!(end.date().day(), 16);

        assert_eq!(end.time().hour(), 18);
        assert_eq!(end.time().minute(), 23);
        assert_eq!(end.time().second(), 0);
    }

    #[test]
    #[cfg(feature = "second-precision")]
    fn from_str_minute_precision_encountered_when_second_precision_expected() {
        let t = Activity::from_str(
            "2021-02-16 16:14 - 2021-02-16 18:23 | test project | test description",
        )
        .unwrap();

        assert_eq!(t.start.date().year(), 2021);
        assert_eq!(t.start.date().month(), 2);
        assert_eq!(t.start.date().day(), 16);

        assert_eq!(t.start.time().hour(), 16);
        assert_eq!(t.start.time().minute(), 14);
        assert_eq!(t.start.time().second(), 0);

        assert_ne!(t.end, None);

        let end = t.end.unwrap();

        assert_eq!(end.date().year(), 2021);
        assert_eq!(end.date().month(), 2);
        assert_eq!(end.date().day(), 16);

        assert_eq!(end.time().hour(), 18);
        assert_eq!(end.time().minute(), 23);
        assert_eq!(end.time().second(), 0);
    }
}
