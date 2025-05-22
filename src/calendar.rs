use crate::task::Task;

use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_reader};
use std::fs::File;
use std::io::{Write, BufReader};

#[derive(Serialize, Deserialize)]
pub struct Calendar (pub Vec<Task>);

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
            return;
        }
        for task in calendar_vec {
            println!("{} [󰃰 : {}]", task.name, task.due_date);
            for subtask in task.subtasks.iter() {
                println!("  * subtask: {} [ : {}]", subtask.name, subtask.days_required.0);
            }
            println!();
        };
    }

    pub fn print_tasks(&self) {
        let Calendar(calendar_vec) = self;
        if calendar_vec.is_empty() {
            println!("you have no upcoming tasks!");
            return;
        }
        for (i, task) in calendar_vec.iter().enumerate() {
            println!("{}) {} [󰃰 : {}]", i, task.name, task.due_date);
        };
    }

    fn to_json(&self) -> Result<String, String> {
        let calendar_ser = to_string_pretty(self).map_err(|_| "Error, failed to serialize data, all changes were lost")?;
        Ok(calendar_ser)
    }

    pub fn save(&self) -> Result<(), String> {
        let mut file: File = File::create("test_cal.json").map_err(|_| "Error, failed to create json file, all changes were lost")?;
        let json: String = self.to_json()?;
        writeln!(file, "{json}").map_err(|_| "Error, failed to write to json file, all changes were lost")?;
        Ok(())
    }

    pub fn load() -> Result<Calendar, String> {
        match File::open("test_cal.json") {
            Ok(file) => {
                let reader = BufReader::new(file);
                let new_calendar = from_reader(reader).map_err(|_| "Error, failed to load calendar")?;
                Ok(new_calendar)
            },
            Err(_) => Ok(Calendar::new())
        }
    }
}
