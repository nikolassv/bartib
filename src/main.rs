use anyhow::{bail, Context, Result};
use chrono::{Duration, Local, NaiveDate, NaiveTime};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

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
        .conflicts_with_all(&["from_date", "to_date", "date", "yesterday"])
        .takes_value(false);

    let arg_yesterday = Arg::with_name("yesterday")
        .long("yesterday")
        .help("show yesterdays activities")
        .required(false)
        .conflicts_with_all(&["from_date", "to_date", "date", "today"])
        .takes_value(false);

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
        .version("1.0.0")
        .author("Nikolas Schmidt-Voigt <nikolas.schmidt-voigt@posteo.de>")
        .about("A simple timetracker")
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
                .arg(
                    arg_project
                        .clone()
                        .required(true)
                )
                .arg(
                    arg_description
                        .clone()
                        .required(true)
                )
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
                        .default_value("0")
                )
                .arg(&arg_time),
        )
        .subcommand(
            SubCommand::with_name("stop")
                .about("stops all currently running activities")
                .arg(&arg_time),
        )
        .subcommand(
            SubCommand::with_name("cancel")
                .about("cancels all currently running activities")
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
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT")
                        .help("do list activities for this project only")
                        .takes_value(true)
                        .required(false)
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
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .long("project")
                        .value_name("PROJECT")
                        .help("do report activities for this project only")
                        .takes_value(true)
                        .required(false)
                )
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
                        .default_value("10")
                )
        )
        .subcommand(SubCommand::with_name("projects").about("list all projects"))
        .subcommand(
            SubCommand::with_name("edit")
                .about("opens the activity log in an editor")
                .arg(
                    Arg::with_name("editor")
                        .short("e")
                        .value_name("editor")
                        .help("the command to start your prefered text editor")
                        .env("EDITOR")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("check").about("checks file and reports parsing errors"))
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
                .map(|t| Local::today().naive_local().and_time(t));

            bartib::controller::manipulation::start(file_name, project_name, activity_description, time)
        }
        ("continue", Some(sub_m)) => {
            let project_name = sub_m.value_of("project");
            let activity_description = sub_m.value_of("description");
            let time = get_time_argument_or_ignore(sub_m.value_of("time"), "-t/--time")
                .map(|t| Local::today().naive_local().and_time(t));
            let number = get_number_argument_or_ignore(
                sub_m.value_of("number"),
                "-n/--number",
            ).unwrap_or(0);

            bartib::controller::manipulation::continue_last_activity(file_name, project_name, activity_description, time, number)
        }
        ("stop", Some(sub_m)) => {
            let time = get_time_argument_or_ignore(sub_m.value_of("time"), "-t/--time")
                .map(|t| Local::today().naive_local().and_time(t));

            bartib::controller::manipulation::stop(file_name, time)
        }
        ("cancel", Some(_)) => bartib::controller::manipulation::cancel(file_name),
        ("current", Some(_)) => bartib::controller::list::list_running(file_name),
        ("list", Some(sub_m)) => {
            let mut filter = bartib::data::getter::ActivityFilter {
                number_of_activities: get_number_argument_or_ignore(
                    sub_m.value_of("number"),
                    "-n/--number",
                ),
                from_date: get_date_argument_or_ignore(sub_m.value_of("from_date"), "--from"),
                to_date: get_date_argument_or_ignore(sub_m.value_of("to_date"), "--to"),
                date: get_date_argument_or_ignore(sub_m.value_of("date"), "-d/--date"),
                project: sub_m.value_of("project")
            };

            if sub_m.is_present("today") {
                filter.date = Some(Local::now().naive_local().date());
            }

            if sub_m.is_present("yesterday") {
                filter.date = Some(Local::now().naive_local().date() - Duration::days(1));
            }

            let do_group_activities = !sub_m.is_present("no_grouping") && filter.date.is_none();
            bartib::controller::list::list(file_name, filter, do_group_activities)
        }
        ("report", Some(sub_m)) => {
            let mut filter = bartib::data::getter::ActivityFilter {
                number_of_activities: None,
                from_date: get_date_argument_or_ignore(sub_m.value_of("from_date"), "--from"),
                to_date: get_date_argument_or_ignore(sub_m.value_of("to_date"), "--to"),
                date: get_date_argument_or_ignore(sub_m.value_of("date"), "-d/--date"),
                project: sub_m.value_of("project")
            };

            if sub_m.is_present("today") {
                filter.date = Some(Local::now().naive_local().date());
            }

            if sub_m.is_present("yesterday") {
                filter.date = Some(Local::now().naive_local().date() - Duration::days(1));
            }

            bartib::controller::report::show_report(file_name, filter)
        }
        ("projects", Some(_)) => bartib::controller::list::list_projects(file_name),
        ("last", Some(sub_m)) => {
            let number = get_number_argument_or_ignore(
                sub_m.value_of("number"),
                "-n/--number",
            ).unwrap_or(10);
            bartib::controller::list::list_last_activities(file_name, number)
        }
        ("edit", Some(sub_m)) => {
            let optional_editor_command = sub_m.value_of("editor");
            bartib::controller::manipulation::start_editor(file_name, optional_editor_command)
        }
        ("check", Some(_)) => bartib::controller::list::check(file_name),
        _ => bail!("Unknown command"),
    }
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
                "Can not parse \"{}\" as number. Argument for {} is ignored",
                number_string, argument_name
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
                    "Can not parse \"{}\" as date. Argument for {} is ignored ({})",
                    date_string, argument_name, parsing_error
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
                    "Can not parse \"{}\" as time. Argument for {} is ignored ({})",
                    time_string, argument_name, parsing_error
                );
                None
            }
        }
    } else {
        None
    }
}
