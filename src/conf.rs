use chrono::Duration;

#[cfg(feature = "second-precision")]
pub static FORMAT_DATETIME: &str = "%F %T";
#[cfg(not(feature = "second-precision"))]
pub static FORMAT_DATETIME: &str = "%F %R";

#[cfg(feature = "second-precision")]
pub static FORMAT_TIME: &str = "%T";
#[cfg(not(feature = "second-precision"))]
pub static FORMAT_TIME: &str = "%R";

pub static FORMAT_DATE: &str = "%F";
pub static DEFAULT_WIDTH: usize = usize::MAX;
pub static REPORT_INDENTATION: usize = 4;

#[derive(Debug)]
pub struct ProcessConfig {
    pub round: Option<Duration>,
}
