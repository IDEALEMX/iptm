pub mod date;
pub mod task;
pub mod subtask;
pub mod day;
pub mod calendar;
pub mod input;

use calendar::Calendar;
use input::{get_task, get_subtask, get_number};
use task::Task;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("new") => {
            handle_new(args);
        },

        Some("list") => {
            let calendar: Calendar = Calendar::load().unwrap();
            calendar.print();
            calendar.save().unwrap();
        }

        None => eprintln!("Error, missing arguments"),

        _ => eprintln!("Error, unrecognized arguments"),
    }
}

fn handle_new(args: Vec<String>) {
    match args.get(2).map(String::as_str) {
        Some("task") => {
            let mut calendar: Calendar = Calendar::load().unwrap();
            let new_task: Task = get_task().unwrap();
            calendar.push(new_task);
            calendar.save().unwrap();
        },

        Some("subtask") => {
            let mut calendar: Calendar = Calendar::load().unwrap();
            calendar.print_tasks();
            let Calendar(vec) = &mut calendar;
            if vec.is_empty() {
                return;
            }
            let task_index: usize = get_number("Enter parent task index: ").unwrap();
            let parent_task: &mut Task = vec.get_mut(task_index).expect("Error, invalid task index");
            let subtask = get_subtask().unwrap();
            parent_task.push(subtask);
            calendar.save().unwrap();
        }

        None => eprintln!("Error, missing arguments after new"),

        _ => eprintln!("Error, unrecognized arguments after new"),
    }
}

