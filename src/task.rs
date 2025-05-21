use crate::calendar::Calendar;
use crate::id::Id;

use std::path::{Path, PathBuf};
use chrono::NaiveDate;

pub struct Task {
    pub id: Id,
    pub name: String,
    pub due_date: NaiveDate,
    pub details_file: Box<Path>,
    pub related_files: Vec<Box<Path>>,
    pub subtasks: Vec<u32>,
    pub difficulty: Option<u8>,
    pub is_subtask: bool
}

impl Task {
    pub fn from(calendar: Calendar, name: &str, due_date: NaiveDate, difficulty: Option<u8>, is_subtask: bool) -> Option<Task> {

        if let Some(diff) = difficulty {    
            if diff > 10 || diff == 0 { 
                eprintln!("Error, difficulty but be in a 1-10 range");
                return None;
            }
        }

        let id: Id = Id::new(calendar);
        let path: String = format!("{}/.local/share/iptm/details_files/", std::env::var("HOME").unwrap());
        let details_file: Box<Path> = PathBuf::from(path).into_boxed_path();
    
        let new_task: Task = Task {
            id, 
            name: name.to_string(),
            due_date,
            details_file,
            related_files: Vec::new(),
            subtasks: Vec::new(),
            difficulty,
            is_subtask
        };

        Some(new_task)
    }
}

