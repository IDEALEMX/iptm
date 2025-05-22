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
        };

        Ok(new_task)
    }

    pub fn push(&mut self, subtask: Subtask) {
        self.subtasks.push(subtask);
    }
}

