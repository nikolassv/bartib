use std::fmt;

use chrono::Duration;
use nu_ansi_term::{Color, Style};

use crate::data::activity;
use crate::data::processor::{StatusReportData, StatusReportWriter};
use crate::view::format_util;

pub struct StatusReport {}

impl StatusReportWriter for StatusReport {
    fn process<'a>(&self, data: &'a StatusReportData) -> anyhow::Result<()> {
        println!("{data}");
        Ok(())
    }
}

impl<'a> fmt::Display for StatusReportData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let longest_line = 30;
        print_title(f, self.project)?;
        print_activity(f, self.activity, self.project)?;
        print_duration(f, "Today", self.today, longest_line)?;
        print_duration(f, "Current week", self.current_week, longest_line)?;
        print_duration(f, "Current month", self.current_month, longest_line)?;

        Ok(())
    }
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
    match activity {
        Some(activity) => {
            write(f, "\n  NOW: ", Style::new().italic().dimmed())?;
            write(f, activity.description.as_str(), Color::Green.bold())?;
            if project.is_none() {
                write(f, " on ", Style::new().italic().dimmed())?;
                write(f, &activity.project, Style::new().italic())?;
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
