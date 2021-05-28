use clap::{App, AppSettings, Arg, SubCommand};

fn main() {	
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
				.help("the file in wich bartib tracks all the tasks")
				.env("BARTIB_FILE")
				.required(true)
				.takes_value(true)
		)
		.subcommand(
			SubCommand::with_name("start")
				.about("starts a new task")
				.arg(
					Arg::with_name("project")
						.short("p")
						.value_name("PROJECT")
						.help("the project to which the new task belong")
						.required(true)
						.takes_value(true)
				)
				.arg(
					Arg::with_name("description")
						.short("d")
						.value_name("DESCRIPTION")
						.help("a description of the new task")
						.required(true)
						.takes_value(true)
				)
		)
		.subcommand(
			SubCommand::with_name("stop")
				.about("stops all currently running tasks")
		)
		.subcommand(
			SubCommand::with_name("current")
				.about("lists all currently running tasks")
		)
		.subcommand(
			SubCommand::with_name("list")
				.about("list recent tasks")
				.arg(
					Arg::with_name("number")
						.short("n")
						.value_name("NUMBER")
						.help("maximum number of tasks to display")
						.required(false)
						.takes_value(true)
				)
		)
		.get_matches();

	let file_name = matches.value_of("file").unwrap();	
	
	match matches.subcommand() {
		("start", Some(sub_m)) => {		
			let project_name = sub_m.value_of("project").unwrap();
			let task_description = sub_m.value_of("description").unwrap();
					
			bartib::start(file_name, project_name, task_description);

		},
		("stop", Some(_)) => bartib::stop(file_name),
		("current", Some(_)) => bartib::list_running(file_name),
		("list", Some(sub_m)) => {
			let number_of_tasks : Option<usize> = get_number_argument_or_ignore(sub_m.value_of("number"), "-n/--number");
			bartib::list(file_name, number_of_tasks)
		},
		_ => println!("Unknown command")
	}
}

fn get_number_argument_or_ignore(number_argument : Option<&str>, argument_name : &str) -> Option<usize> {
	if let Some(number_string) = number_argument {
		let parsing_result = number_string.parse();
		
		if let Ok(number) = parsing_result {
			Some(number)
		} else {
			println!("Can not parse \"{}\" as number. Argument for {} is ignored", number_string, argument_name);
			None
		}
	} else {
		None
	}
}
