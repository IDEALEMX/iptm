pub mod date;
pub mod task;
pub mod subtask;
pub mod day;
pub mod calendar;
pub mod input;

use calendar::Calendar;
use input::{get_task, get_subtask, get_number};
use task::Task;
use std::process::Command;
use std::fs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("new") => {
            handle_new(args);
        },

        Some("read") => {
            handle_read(args);
        }

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

fn handle_read(args: Vec<String>) {
    match args.get(2).map(String::as_str) {
        Some("related") => {
        },

        Some("task") => {
            let calendar: Calendar = Calendar::load().unwrap();
            calendar.print_tasks();
            let Calendar(vec) = calendar;
            if vec.is_empty() {
                return;
            }
            let task_index: usize = get_number("Enter parent task index: ").unwrap();
            let task: &Task = vec.get(task_index).expect("Error, invalid task index");

            let dir = format!("{}/.local/share/iptm/details_files/", std::env::var("HOME").expect("Error, failed to access user home"));

            fs::create_dir_all(&dir).expect("Error: failed to create directory");

            Command::new("/home/ideale/.nixvim/result/bin/nvim")
                .arg(task.details_file.to_str().expect("Error, failed to convert related file"))
                .status()
                .expect("Error, failed to load editor");
        },

        Some("subtask") => {
        },

        None => eprintln!("Error, missing arguments after new"),

        _ => eprintln!("Error, unrecognized arguments after new"),
    }
}

