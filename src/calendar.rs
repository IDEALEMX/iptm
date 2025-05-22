use crate::task::Task;

use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_reader};
use std::fs::File;
use std::io::{Write, BufReader};

#[derive(Serialize, Deserialize)]
pub struct Calendar (Vec<Task>);

impl Calendar {
    fn new() -> Calendar {
        let calendar:Vec<Task> = Vec::new();
        Calendar(calendar)
    }

    pub fn push(&mut self, new_task: Task) {
        let Calendar(calendar_vec) = self;
        calendar_vec.push(new_task);
    }

    pub fn print(&self) {
        let Calendar(calendar_vec) = self;
        if calendar_vec.is_empty() {
            println!("you have no upcoming tasks!");
        }
        for task in calendar_vec {
            println!("task: {}, due: {}", task.name, task.due_date);
            for subtask in task.subtasks.iter() {
                println!("  * subtask: {}", subtask.name);
            }
        };
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        let calendar_ser: Result<String, serde_json::Error> = to_string_pretty(self);
        calendar_ser
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file: File = File::create("test_cal.json")?;
        let json: String = self.to_json()?;
        writeln!(file, "{json}")?;
        Ok(())
    }

    pub fn load() -> Result<Calendar, Box<dyn std::error::Error>> {
        match File::open("test_cal.json") {
            Ok(file) => {
                let reader = BufReader::new(file);
                let new_calendar = from_reader(reader)?;
                Ok(new_calendar)
            },
            Err(_) => Ok(Calendar::new())
        }
    }
}
