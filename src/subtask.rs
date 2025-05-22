use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::day::Day;

#[derive(Serialize, Deserialize)]
pub struct Subtask {
    pub id: Uuid,
    pub name: String,
    pub details_file: Box<Path>,
    pub days_required: Day
}

impl Subtask {
    pub fn new(name: &str, days_required: Day) -> Result<Subtask, String> {
        let id: Uuid = Uuid::new_v4();
        let path: String = format!("{}/.local/share/iptm/details_files/{}.md", std::env::var("HOME").map_err(|_| "Error, failed to access user home")?, id);
        let details_file: Box<Path> = PathBuf::from(path).into_boxed_path();
    
        let new_task: Subtask = Subtask {
            id,
            name: name.to_string(),
            details_file,
            days_required,
        };

        Ok(new_task)
    }
}
