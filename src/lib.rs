mod conf;
mod project;
mod task;
mod table;
mod format_util;
pub mod bartib_file;

pub fn start(file_name: &str, project_name: &str, task_description: &str) {
	let mut file_content = bartib_file::get_file_content(file_name);
	
	stop_all_running_tasks(&mut file_content);
	
	let project = project::Project(project_name.to_string());
	let task = task::Task::start(project, task_description.to_string());	
	file_content.push(bartib_file::Line::for_task(task));
	
	bartib_file::write_to_file(file_name, &file_content).expect("Could not write to file");
}

pub fn stop(file_name: &str) {
	let mut file_content = bartib_file::get_file_content(file_name);	
	stop_all_running_tasks(&mut file_content);	
	bartib_file::write_to_file(file_name, &file_content).expect("Could not write to file");
}

pub fn list_running(file_name: &str) {
	let file_content = bartib_file::get_file_content(file_name);
	let running_tasks = get_running_tasks(&file_content);
	list_running_tasks(running_tasks);
}

fn stop_all_running_tasks(file_content: &mut [bartib_file::Line]) {
	for line in file_content {
		if let Ok(task) = &mut line.task {
			if !task.is_stopped() {
				task.stop();
				line.set_changed();
			}
		}
	}
}

fn get_running_tasks(file_content: &[bartib_file::Line]) -> Vec<&task::Task> {
	file_content.iter()
		.map(|line| line.task.as_ref())
		.filter_map(|task_result| task_result.ok())
		.filter(|task| !task.is_stopped())
		.collect()
}

fn list_running_tasks(running_tasks: Vec<&task::Task>) {
	if running_tasks.is_empty() {
		println!("No Task is currently running");
	} else {		
		let mut task_table = table::Table::new(vec!["Started At", "Description", "Project", "Duration"]);
		
		running_tasks.iter()
			.map(|task| table::Row::new(vec![
				task.start.format(conf::FORMAT_DATETIME).to_string(),
				task.description.clone(),
				task.project.to_string(),
				format_util::format_duration(&task.get_duration())
			]))
			.for_each(|row| task_table.add_row(row));
			
		println!("\n{}", task_table);
	}
}