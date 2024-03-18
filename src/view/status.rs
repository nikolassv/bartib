use std::fmt;

use chrono::Duration;
use nu_ansi_term::{Color, Style};

use crate::conf;
use crate::data::activity;
use crate::view::format_util;

pub struct StatusReportData<'a> {
    pub activity: Option<&'a activity::Activity>,
    pub project: Option<&'a str>,
    pub today: Duration,
    pub current_week: Duration,
    pub current_month: Duration,
}

struct Report<'a> {
    data: StatusReportData<'a>,
}

impl<'a> Report<'a> {
    fn new(data: StatusReportData<'a>) -> Report<'a> {
        Report { data }
    }
}

impl<'a> fmt::Display for Report<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut longest_line = 30;
        let terminal_width = term_size::dimensions_stdout().map_or(conf::DEFAULT_WIDTH, |d| d.0);

        if terminal_width <= longest_line {
            longest_line = terminal_width;
        }

        print_title(f, self.data.project)?;
        print_activity(f, self.data.activity, self.data.project)?;
        print_duration(f, "Today", self.data.today, longest_line)?;
        print_duration(f, "Current week", self.data.current_week, longest_line)?;
        print_duration(f, "Current month", self.data.current_month, longest_line)?;

        Ok(())
    }
}

pub fn show(data: StatusReportData) {
    let report = Report::new(data);
    println!("{report}");
}

fn print_duration(
    f: &mut fmt::Formatter<'_>,
    name: &str,
    total_duration: Duration,
    line_width: usize,
) -> fmt::Result {
    write(f, " ", Style::new().italic())?;
    write_period(f, name, line_width, Style::new().italic().dimmed())?;
    write(
        f,
        format_util::format_duration(&total_duration).as_str(),
        Style::new().bold(),
    )?;
    write(f, "\n", Style::new().italic())?;
    Ok(())
}

fn print_activity(
    f: &mut fmt::Formatter<'_>,
    activity: Option<&activity::Activity>,
    project: Option<&str>,
) -> fmt::Result {
    let activity_style = Color::Green.bold();
    let project_style = Style::new().italic();
    match activity {
        Some(activity) => {
            write(f, "\n  NOW: ", Style::new().italic().dimmed())?;
            write(f, activity.description.as_str(), activity_style)?;
            if let Some(pr_str) = project {
                write(f, " on ", Style::new().italic().dimmed())?;
                write(f, pr_str, project_style)?;
            };
            write(f, " ...... ", Style::new().dimmed())?;
            write(
                f,
                format_util::format_duration(&activity.get_duration()).as_str(),
                Style::new().bold(),
            )?;
            write(f, "\n\n", Style::new().dimmed())?;
        }
        None => {
            write(f, "\n  NOW: ", Style::new().italic().dimmed())?;
            write(f, " NO Activity\n\n", Style::new().bold())?;
        }
    }
    Ok(())
}

fn print_title(f: &mut fmt::Formatter<'_>, project: Option<&str>) -> fmt::Result {
    match project {
        Some(project) => {
            write(f, "\n =======", Style::new().dimmed())?;
            write(f, " Status for project: ", Style::new().italic())?;
            write(f, project, Style::new().bold())?;
        }
        None => {
            write(f, "\n =======", Style::new().dimmed())?;
            write(f, " Status for ", Style::new().italic())?;
            write(f, "ALL", Style::new().bold())?;
            write(f, " projects ", Style::new().italic())?;
        }
    }
    write(f, " ======= \n", Style::new().dimmed())?;
    Ok(())
}

fn write(f: &mut fmt::Formatter<'_>, text: &str, style: Style) -> fmt::Result {
    write!(
        f,
        "{prefix}{text}{suffix}",
        prefix = style.prefix(),
        suffix = style.infix(Style::new())
    )?;
    Ok(())
}

fn write_period(
    f: &mut fmt::Formatter<'_>,
    text: &str,
    line_width: usize,
    style: Style,
) -> fmt::Result {
    write!(
        f,
        "{prefix} {text:.<line_width$} {suffix}",
        prefix = style.prefix(),
        suffix = style.infix(Style::new())
    )?;
    Ok(())
}
