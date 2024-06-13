use std::borrow::Borrow;

use anyhow::{bail, Context, Result};
use bartib::view::status::StatusReport;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};

use bartib::data::getter::ActivityFilter;
use bartib::data::processor;

#[cfg(windows)]
use nu_ansi_term::enable_ansi_support;

fn main() -> Result<()> {
    #[cfg(windows)]
    if let Err(e) = enable_ansi_support() {
        println!("Could not enable ansi support! Errorcode: {}", e);
    }

    let arg_time = Arg::with_name("time")
        .short("t")
        .long("time")
        .value_name("TIME")
        .help("the time for changing the activity status (HH:MM)")
        .takes_value(true);

    let arg_from_date = Arg::with_name("from_date")
        .long("from")
        .value_name("FROM_DATE")
        .help("begin of date range (inclusive)")
        .takes_value(true);

    let arg_to_date = Arg::with_name("to_date")
        .long("to")
        .value_name("TO_DATE")
        .help("end of date range (inclusive)")
        .takes_value(true);

    let arg_date = Arg::with_name("date")
        .short("d")
        .long("date")
        .value_name("DATE")
        .help("show activities of a certain date only")
        .required(false)
        .conflicts_with_all(&["from_date", "to_date"])
        .takes_value(true);

    let arg_today = Arg::with_name("today")
        .long("today")
        .help("show activities of the current day")
        .required(false)
        .conflicts_with_all(&["from_date", "to_date", "date"])
        .takes_value(false);

    let arg_yesterday = Arg::with_name("yesterday")
        .long("yesterday")
        .help("show yesterdays' activities")
        .required(false)
        .conflicts_with_all(&["from_date", "to_date", "date", "today"])
        .takes_value(false);

    let arg_current_week = Arg::with_name("current_week")
        .long("current_week")
        .help("show activities of the current week")
        .required(false)
        .conflicts_with_all(&["from_date", "to_date", "date", "today", "yesterday"])
        .takes_value(false);

    let arg_last_week = Arg::with_name("last_week")
        .long("last_week")
        .help("show activities of the last week")
        .required(false)
        .conflicts_with_all(&[
            "from_date",
            "to_date",
            "date",
            "today",
            "yesterday",
            "current_week",
        ])
        .takes_value(false);

    let arg_group = Arg::with_name("round")
        .long("round")
        .help("rounds the start and end time to the nearest duration. Durations can be in minutes or hours. E.g. 15m or 4h")
        .required(false)
        .takes_value(true);

    let arg_description = Arg::with_name("description")
        .short("d")
        .long("description")
        .value_name("DESCRIPTION")
        .help("the description of the new activity")
        .takes_value(true);

    let arg_project = Arg::with_name("project")
        .short("p")
        .long("project")
        .value_name("PROJECT")
        .help("the project to which the new activity belongs")
        .takes_value(true);

    let matches = App::new("bartib")
        .version(crate_version!())
        .author("Nikolas Schmidt-Voigt <nikolas.schmidt-voigt@posteo.de>")
        .about("A simple timetracker")
        .after_help("To get help for a specific subcommand, run `bartib [SUBCOMMAND] --help`.
To get started, view the `start` help with `bartib start --help`")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .help("the file in which bartib tracks all the activities")
                .env("BARTIB_FILE")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("starts a new activity")
                .arg(arg_project.clone().required(true))
                .arg(arg_description.clone().required(true))
                .arg(&arg_time),
        )
        .subcommand(
            SubCommand::with_name("continue")
                .about("continues a previous activity")
                .arg(&arg_description)
                .arg(&arg_project)
                .arg(
                    Arg::with_name("number")
                        .value_name("NUMBER")
                        .help("the number of the activity to continue (see subcommand `last`)")
                        .required(false)
                        .takes_value(true)
                        .default_value("0"),
                )
                .arg(&arg_time),
        )
        .subcommand(
            SubCommand::with_name("change")
                .about("changes the current activity")
                .arg(&arg_description)
                .arg(&arg_project)
                .arg(&arg_time),
        )
        .subcommand(
            SubCommand::with_name("stop")
                .about("stops all currently running activities")
                .arg(&arg_time),
        )
        .subcommand(
            SubCommand::with_name("cancel").about("cancels all currently running activities"),
        )
        .subcommand(
            SubCommand::with_name("current").about("lists all currently running activities"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("list recent activities")
                .arg(&arg_from_date)
                .arg(&arg_to_date)
                .arg(&arg_date)
                .arg(&arg_today)
                .arg(&arg_yesterday)
                .arg(&arg_current_week)
                .arg(&arg_last_week)
                .arg(&arg_group)
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT")
                        .help("do list activities for this project only")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("no_grouping")
                        .long("no_grouping")
                        .help("do not group activities by date in list"),
                )
                .arg(
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .value_name("NUMBER")
                        .help("maximum number of activities to display")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("report")
                .about("reports duration of tracked activities")
                .arg(&arg_from_date)
                .arg(&arg_to_date)
                .arg(&arg_date)
                .arg(&arg_today)
                .arg(&arg_yesterday)
                .arg(&arg_current_week)
                .arg(&arg_last_week)
                .arg(&arg_group)
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT")
                        .help("do report activities for this project only")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("last")
                .about("displays the descriptions and projects of recent activities")
                .arg(
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .value_name("NUMBER")
                        .help("maximum number of lines to display")
                        .required(false)
                        .takes_value(true)
                        .default_value("10"),
                ),
        )
        .subcommand(
            SubCommand::with_name("projects")
                .about("list all projects")
                .arg(
                    Arg::with_name("current")
                        .short("c")
                        .long("current")
                        .help("prints currently running projects only")
                        .takes_value(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("no-quotes")
                        .short("n")
                        .long("no-quotes")
                        .help("prints projects without quotes")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("opens the activity log in an editor")
                .arg(
                    Arg::with_name("editor")
                        .short("e")
                        .value_name("editor")
                        .help("the command to start your preferred text editor")
                        .env("EDITOR")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("check").about("checks file and reports parsing errors"))
        .subcommand(SubCommand::with_name("sanity").about("checks sanity of bartib log"))
        .subcommand(SubCommand::with_name("search").about("search for existing descriptions and projects")
                .arg(
                    Arg::with_name("search_term")
                        .value_name("SEARCH_TERM")
                        .help("the search term")
                        .required(true)
                        .takes_value(true)
                        .default_value("''"),
                ),
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("shows current status and time reports for today, current week, and current month")
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT")
                        .help("show status for this project only")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .get_matches();

    let file_name = matches.value_of("file")
        .context("Please specify a file with your activity log either as -f option or as BARTIB_FILE environment variable")?;

    run_subcommand(&matches, file_name)
}

fn run_subcommand(matches: &ArgMatches, file_name: &str) -> Result<()> {
    match matches.subcommand() {
        ("start", Some(sub_m)) => {
            let project_name = sub_m.value_of("project").unwrap();
            let activity_description = sub_m.value_of("description").unwrap();
            let time = get_time_argument_or_ignore(sub_m.value_of("time"), "-t/--time")
                .map(|t| Local::now().date_naive().and_time(t));

            bartib::controller::manipulation::start(
                file_name,
                project_name,
                activity_description,
                time,
            )
        }
        ("change", Some(sub_m)) => {
            let project_name = sub_m.value_of("project");
            let activity_description = sub_m.value_of("description");
            let time = get_time_argument_or_ignore(sub_m.value_of("time"), "-t/--time")
                .map(|t| Local::now().date_naive().and_time(t));

            bartib::controller::manipulation::change(
                file_name,
                project_name,
                activity_description,
                time,
            )
        }
        ("continue", Some(sub_m)) => {
            let project_name = sub_m.value_of("project");
            let activity_description = sub_m.value_of("description");
            let time = get_time_argument_or_ignore(sub_m.value_of("time"), "-t/--time")
                .map(|t| Local::now().date_naive().and_time(t));
            let number =
                get_number_argument_or_ignore(sub_m.value_of("number"), "-n/--number").unwrap_or(0);

            bartib::controller::manipulation::continue_last_activity(
                file_name,
                project_name,
                activity_description,
                time,
                number,
            )
        }
        ("stop", Some(sub_m)) => {
            let time = get_time_argument_or_ignore(sub_m.value_of("time"), "-t/--time")
                .map(|t| Local::now().date_naive().and_time(t));

            bartib::controller::manipulation::stop(file_name, time)
        }
        ("cancel", Some(_)) => bartib::controller::manipulation::cancel(file_name),
        ("current", Some(_)) => bartib::controller::list::list_running(file_name),
        ("list", Some(sub_m)) => {
            let filter = create_filter_for_arguments(sub_m);
            let processors = create_processors_for_arguments(sub_m);
            let do_group_activities = !sub_m.is_present("no_grouping") && filter.date.is_none();
            bartib::controller::list::list(file_name, filter, do_group_activities, processors)
        }
        ("report", Some(sub_m)) => {
            let filter = create_filter_for_arguments(sub_m);
            let processors = create_processors_for_arguments(sub_m);
            bartib::controller::report::show_report(file_name, filter, processors)
        }
        ("projects", Some(sub_m)) => bartib::controller::list::list_projects(
            file_name,
            sub_m.is_present("current"),
            sub_m.is_present("no-quotes"),
        ),
        ("last", Some(sub_m)) => {
            let number = get_number_argument_or_ignore(sub_m.value_of("number"), "-n/--number")
                .unwrap_or(10);
            bartib::controller::list::list_last_activities(file_name, number)
        }
        ("edit", Some(sub_m)) => {
            let optional_editor_command = sub_m.value_of("editor");
            bartib::controller::manipulation::start_editor(file_name, optional_editor_command)
        }
        ("check", Some(_)) => bartib::controller::list::check(file_name),
        ("sanity", Some(_)) => bartib::controller::list::sanity_check(file_name),
        ("search", Some(sub_m)) => {
            let search_term = sub_m.value_of("search_term");
            bartib::controller::list::search(file_name, search_term)
        }
        ("status", Some(sub_m)) => {
            let filter = create_filter_for_arguments(sub_m);
            let processors = create_processors_for_arguments(sub_m);
            let writer = create_status_writer(sub_m);
            bartib::controller::status::show_status(file_name, filter, processors, writer.borrow())
        }
        _ => bail!("Unknown command"),
    }
}

fn create_processors_for_arguments(sub_m: &ArgMatches) -> processor::ProcessorList {
    let mut processors: Vec<Box<dyn processor::ActivityProcessor>> = Vec::new();

    if let Some(round) = get_duration_argument_or_ignore(sub_m.value_of("round"), "--round") {
        processors.push(Box::new(processor::RoundProcessor { round }));
    }

    processors
}

fn create_status_writer(_sub_m: &ArgMatches) -> Box<dyn processor::StatusReportWriter> {
    let result = StatusReport {};
    Box::new(result)
}

fn create_filter_for_arguments<'a>(sub_m: &'a ArgMatches) -> ActivityFilter<'a> {
    let mut filter = ActivityFilter {
        number_of_activities: get_number_argument_or_ignore(
            sub_m.value_of("number"),
            "-n/--number",
        ),
        from_date: get_date_argument_or_ignore(sub_m.value_of("from_date"), "--from"),
        to_date: get_date_argument_or_ignore(sub_m.value_of("to_date"), "--to"),
        date: get_date_argument_or_ignore(sub_m.value_of("date"), "-d/--date"),
        project: sub_m.value_of("project"),
    };

    let today = Local::now().naive_local().date();
    if sub_m.is_present("today") {
        filter.date = Some(today);
    }

    if sub_m.is_present("yesterday") {
        filter.date = Some(today - Duration::days(1));
    }

    if sub_m.is_present("current_week") {
        filter.from_date =
            Some(today - Duration::days(i64::from(today.weekday().num_days_from_monday())));
        filter.to_date = Some(
            today - Duration::days(i64::from(today.weekday().num_days_from_monday()))
                + Duration::days(6),
        );
    }

    if sub_m.is_present("last_week") {
        filter.from_date = Some(
            today
                - Duration::days(i64::from(today.weekday().num_days_from_monday()))
                - Duration::weeks(1),
        );
        filter.to_date = Some(
            today
                - Duration::days(i64::from(today.weekday().num_days_from_monday()))
                - Duration::weeks(1)
                + Duration::days(6),
        )
    }

    filter
}

fn get_number_argument_or_ignore(
    number_argument: Option<&str>,
    argument_name: &str,
) -> Option<usize> {
    if let Some(number_string) = number_argument {
        let parsing_result = number_string.parse();

        if let Ok(number) = parsing_result {
            Some(number)
        } else {
            println!(
                "Can not parse \"{number_string}\" as number. Argument for {argument_name} is ignored"
            );
            None
        }
    } else {
        None
    }
}

fn get_date_argument_or_ignore(
    date_argument: Option<&str>,
    argument_name: &str,
) -> Option<NaiveDate> {
    if let Some(date_string) = date_argument {
        let parsing_result = NaiveDate::parse_from_str(date_string, bartib::conf::FORMAT_DATE);

        match parsing_result {
            Ok(date) => Some(date),
            Err(parsing_error) => {
                println!(
                    "Can not parse \"{date_string}\" as date. Argument for {argument_name} is ignored ({parsing_error})"
                );
                None
            }
        }
    } else {
        None
    }
}

fn get_time_argument_or_ignore(
    time_argument: Option<&str>,
    argument_name: &str,
) -> Option<NaiveTime> {
    if let Some(time_string) = time_argument {
        let parsing_result = NaiveTime::parse_from_str(time_string, bartib::conf::FORMAT_TIME);

        match parsing_result {
            Ok(date) => Some(date),
            Err(parsing_error) => {
                println!(
                    "Can not parse \"{time_string}\" as time. Argument for {argument_name} is ignored ({parsing_error})"
                );
                None
            }
        }
    } else {
        None
    }
}

fn get_duration_argument_or_ignore(
    duration_argument: Option<&str>,
    argument_name: &str,
) -> Option<chrono::Duration> {
    if let Some(duration_string) = duration_argument {
        // extract last character, leave the rest as number
        let (number_string, duration_unit) = duration_string.split_at(duration_string.len() - 1);

        let number: Option<i64> = number_string.parse().ok();

        match number {
            Some(number) => match duration_unit {
                "m" => Some(chrono::Duration::minutes(number)),
                "h" => Some(chrono::Duration::hours(number)),
                _ => {
                    println!(
                            "Can not parse \"{duration_string}\" as duration. Argument for {argument_name} is ignored"
                        );
                    None
                }
            },
            None => None,
        }
    } else {
        None
    }
}
