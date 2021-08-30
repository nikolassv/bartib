use anyhow::{bail, Context, Result};
use chrono::{NaiveDate, Local, Duration};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

#[cfg(windows)]
use nu_ansi_term::enable_ansi_support;

fn main() -> Result<()> {

    #[cfg(windows)]
    if let Err(e) = enable_ansi_support() {
        println!("Could not enable ansi support! Errorcode: {}", e);
    }

    let matches = App::new("bartib")
        .version("0.1")
        .author("Nikolas Schmidt-Voigt <nikolas.schmidt-voigt@posteo.de>")
        .about("A simple timetracker")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .help("the file in wich bartib tracks all the activities")
                .env("BARTIB_FILE")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("starts a new activity")
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .value_name("PROJECT")
                        .help("the project to which the new activity belong")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .value_name("DESCRIPTION")
                        .help("a description of the new activity")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("continue")
                .about("continues the last activity")
                .arg(
                    Arg::with_name("project")
                        .short("p")
                        .value_name("PROJECT")
                        .help("the project to which the new activity belong")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .value_name("DESCRIPTION")
                        .help("a description of the new activity")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("stop").about("stops all currently running activities"))
        .subcommand(
            SubCommand::with_name("current").about("lists all currently running activities"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("list recent activities")
                .arg(
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .value_name("NUMBER")
                        .help("maximum number of activities to display")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("from_date")
                        .long("from")
                        .value_name("FROM_DATE")
                        .help("begin of date range (inclusive)")
                        .requires("to_date")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("to_date")
                        .long("to")
                        .value_name("TO_DATE")
                        .help("end of date range (inclusive)")
                        .requires("from_date")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("date")
                        .short("d")
                        .long("date")
                        .value_name("DATE")
                        .help("show activities of a certain date only")
                        .required(false)
                        .conflicts_with_all(&["from_date", "to_date"])
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("today")
                        .long("today")
                        .help("show activities of the current day")
                        .required(false)
                        .conflicts_with_all(&["from_date", "to_date", "date", "yesterday"])
                        .takes_value(false)
                )
                .arg(
                    Arg::with_name("yesterday")
                        .long("yesterday")
                        .help("show yesterdays activities")
                        .required(false)
                        .conflicts_with_all(&["from_date", "to_date", "date", "today"])
                        .takes_value(false)
                )
                .arg(
                    Arg::with_name("no_grouping")
                        .long("no_grouping")
                        .help("do not group activities by date in list"),
                ),
        )
        .subcommand(
            SubCommand::with_name("last").about("displays last finished acitivity")
        )
        .subcommand(
            SubCommand::with_name("projects").about("list all projects")
        )
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
                )
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

            bartib::start(file_name, project_name, activity_description)
        },
        ("continue", Some(sub_m)) => {
            let project_name = sub_m.value_of("project");
            let activity_description = sub_m.value_of("description");

            bartib::continue_last_activity(file_name, project_name, activity_description)
        },
        ("stop", Some(_)) => bartib::stop(file_name),
        ("current", Some(_)) => bartib::list_running(file_name),
        ("list", Some(sub_m)) => {
            let mut filter = bartib::ActivityFilter {
                number_of_activities: get_number_argument_or_ignore(
                    sub_m.value_of("number"),
                    "-n/--number",
                ),
                from_date: get_date_argument_or_ignore(sub_m.value_of("from_date"), "--from"),
                to_date: get_date_argument_or_ignore(sub_m.value_of("to_date"), "--to"),
                date: get_date_argument_or_ignore(sub_m.value_of("date"), "-d/--date"),
            };

            if sub_m.is_present("today") {
                filter.date = Some(Local::now().naive_local().date());
            }

            if sub_m.is_present("yesterday") {
                filter.date = Some(Local::now().naive_local().date() - Duration::days(1));
            }

            let do_group_activities = !sub_m.is_present("no_grouping") && !filter.date.is_some();
            bartib::list(file_name, filter, do_group_activities)
        },
        ("projects", Some(_)) => bartib::list_projects(file_name),
        ("last", Some(_)) => bartib::display_last_activity(file_name),
        ("edit", Some(sub_m)) => {
            let optional_editor_command = sub_m.value_of("editor");
            bartib::start_editor(file_name, optional_editor_command)
        },
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
