use crate::task::Task;
use crate::id::Id;

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_reader};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader};

#[derive(Serialize, Deserialize)]
pub struct Calendar (HashMap<NaiveDate, Vec<Task>>);

impl Calendar {
    pub fn get_task_by_id(&self, search_id: Id) -> Option<&Task> {
        let Calendar(calendar_hash_map) = self;
        for date in calendar_hash_map.values() {
            for task in date.iter() {
                if task.id == search_id {
                    return Some(task);
                };
            };
        };
        return None;
    }

    fn new() -> Calendar {
        let calendar: HashMap<NaiveDate, Vec<Task>> = HashMap::new();
        Calendar(calendar)
    }

    pub fn add(&mut self, new_task: Task) {
        let date = new_task.due_date;
        let Calendar(calendar_hash_map) = self;
        let date_option: Option<&mut Vec<Task>> = calendar_hash_map.get_mut(&date);
        match date_option {
            Some(task_vec) => task_vec.push(new_task),
            None => {calendar_hash_map.insert(date, vec![new_task]);},
        }
    }

    pub fn print(&self) {
        let Calendar(calendar_hash_map) = self;
        for date in calendar_hash_map.values() {
            for task in date.iter() {
                println!("{}- task: {}, due: {}", task.id.get(), task.name, task.due_date);
            };
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
