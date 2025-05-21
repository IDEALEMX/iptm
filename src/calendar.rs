use crate::task::Task;
use crate::id::Id;

use std::collections::HashMap;
use chrono::NaiveDate;

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
}
