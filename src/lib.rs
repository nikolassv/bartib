mod conf;
mod project;
mod task;
mod table;
mod format_util;
mod output;
pub mod bartib_file;

// starts a new task
pub fn start(file_name: &str, project_name: &str, task_description: &str) {
	let mut file_content = bartib_file::get_file_content(file_name);
	
	// if we start a new tasks programaticly, we stop all other tasks first.
	// However, we must not assume that there is always only one task
	// running as the user may have started tasks manually
	stop_all_running_tasks(&mut file_content);
	
	let project = project::Project(project_name.to_string());
	let task = task::Task::start(project, task_description.to_string());	
	file_content.push(bartib_file::Line::for_task(task));
	
	bartib_file::write_to_file(file_name, &file_content).expect("Could not write to file");
}

// stops all currently running tasks
pub fn stop(file_name: &str) {
	let mut file_content = bartib_file::get_file_content(file_name);	
	stop_all_running_tasks(&mut file_content);	
	bartib_file::write_to_file(file_name, &file_content).expect("Could not write to file");
}

// lists all currently runninng tasks.
pub fn list_running(file_name: &str) {
	let file_content = bartib_file::get_file_content(file_name);
	let running_tasks = get_running_tasks(&file_content);
	output::list_running_tasks(&running_tasks);
}

// lists tracked tasks
//
// the tasks will be ordered chronologically. 
pub fn list(file_name: &str, number_of_tasks: Option<usize>) {
	let file_content = bartib_file::get_file_content(file_name);
	let mut all_tasks : Vec<&task::Task> = get_tasks(&file_content).collect();
	
	all_tasks.sort_by_key(|task| task.start);

	let first_element = get_index_of_first_element(all_tasks.len(), number_of_tasks);
	output::list_tasks(&all_tasks[first_element .. all_tasks.len()]);
}

fn get_index_of_first_element(length: usize, sub: Option<usize>) -> usize {
	if let Some(s) = sub {
		length.saturating_sub(s)
	} else {
		0
	}
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
	get_tasks(file_content)
		.filter(|task| !task.is_stopped())
		.collect()
}

fn get_tasks(file_content: &[bartib_file::Line]) -> impl Iterator<Item = &task::Task> {
	file_content.iter()
		.map(|line| line.task.as_ref())
		.filter_map(|task_result| task_result.ok())
}