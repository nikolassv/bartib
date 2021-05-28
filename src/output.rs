use crate::task;
use crate::format_util;
use crate::table;
use crate::conf;

pub fn list_tasks(tasks: &[&task::Task]) {
	if tasks.is_empty() {
		println!("No task to display");
	} else {
		let mut task_table = table::Table::new(vec!["Started", "Stopped", "Description", "Project", "Duration"]);

		tasks.iter()
			.map(|task| table::Row::new(vec![
				task.start.format(conf::FORMAT_DATETIME).to_string(),
				task.end.map_or_else(|| "-".to_string(), |end| end.format(conf::FORMAT_DATETIME).to_string()),
				task.description.clone(),
				task.project.to_string(),
				format_util::format_duration(&task.get_duration())
			]))
			.for_each(|row| task_table.add_row(row));

		println!("\n{}", task_table);
	}
}

// displays a table with running tasks (no end time)
pub fn list_running_tasks(running_tasks: &[&task::Task]) {
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