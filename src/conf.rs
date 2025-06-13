use chrono::Duration;

pub static FORMAT_MINUTE_PRECISION_DATETIME: &str = "%F %R";
pub static FORMAT_SECOND_PRECISION_DATETIME: &str = "%F %T";

#[cfg(not(feature = "second-precision"))]
pub static FORMAT_DATETIME: &str = FORMAT_MINUTE_PRECISION_DATETIME;
#[cfg(feature = "second-precision")]
pub static FORMAT_DATETIME: &str = FORMAT_SECOND_PRECISION_DATETIME;

#[cfg(not(feature = "second-precision"))]
pub static FORMAT_TIME: &str = "%R";
#[cfg(feature = "second-precision")]
pub static FORMAT_TIME: &str = "%T";

pub static FORMAT_DATE: &str = "%F";
pub static DEFAULT_WIDTH: usize = usize::MAX;
pub static REPORT_INDENTATION: usize = 4;

#[derive(Debug)]
pub struct ProcessConfig {
    pub round: Option<Duration>,
}
