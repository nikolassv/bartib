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
		_ => println!("Unknown command")
	}
}
