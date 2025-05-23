use crate::subtask::Subtask;

use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub due_date: NaiveDate,
    pub details_file: Box<Path>,
    pub related_files: Vec<Box<Path>>,
    pub subtasks: Vec<Subtask>,
    pub finished: bool,
}

impl Task {
    pub fn new(name: &str, due_date: NaiveDate) -> Result<Task, String> {
        let id: Uuid = Uuid::new_v4();
        let path: String = format!("{}/.local/share/iptm/details_files/{}.md", std::env::var("HOME").map_err(|_| "Error, failed to access user home")?, id);
        let details_file: Box<Path> = PathBuf::from(path).into_boxed_path();
    
        let new_task: Task = Task {
            id,
            name: name.to_string(),
            due_date,
            details_file,
            related_files: Vec::new(),
            subtasks: Vec::new(),
            finished: false,
        };

        Ok(new_task)
    }

    pub fn push(&mut self, subtask: Subtask) {
        self.subtasks.push(subtask);
    }

    pub fn print_subtasks(&self) -> Result<(), String> {
        let subtask_vec = &self.subtasks;
        for (i, subtask) in subtask_vec.iter().enumerate() {
            println!("{}) {}{}: [ : {}]", i, subtask_is_finished_display(subtask), subtask.name, subtask.days_required.0);
        }
        Ok(())
    }

}

fn subtask_is_finished_display(subtask: &Subtask) -> &str {
    if subtask.finished {
        " "
    } else {
        " "
    }
}
