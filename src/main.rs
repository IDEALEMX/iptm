pub mod date;
pub mod task;
pub mod subtask;
pub mod day;
pub mod calendar;
pub mod input;

use calendar::Calendar;
use input::{create_task, create_subtask, get_number, get_task, get_subtask};
use task::Task;
use std::process::Command;
use std::fs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("new") => {
            match handle_new(args) {
                Err(e) => eprintln!("{e}"),
                _ => (),
            }
        },

        Some("read") => {
            match handle_read(args) {
                Err(e) => eprintln!("{e}"),
                _ => (),
            }
        }

        Some("list") => {
            let calendar: Calendar;
            match Calendar::load() {
                Ok(value) => calendar = value,
                Err(e) => {eprint!("{e}"); return;},
            }
            calendar.print();
            match calendar.save() {
                Ok(_) => (),
                Err(e) => eprintln!("{e}"),
            }
        }

        None => eprintln!("Error, missing arguments"),

        _ => eprintln!("Error, unrecognized arguments"),
    }
}

fn handle_new(args: Vec<String>) -> Result<(), String>{
    match args.get(2).map(String::as_str) {
        Some("task") => {
            let mut calendar: Calendar = Calendar::load()?;
            let new_task: Task = create_task()?;
            calendar.push(new_task);
            calendar.save()
        },

        Some("subtask") => {
            let mut calendar: Calendar = Calendar::load()?;

            calendar.print_tasks();
            let Calendar(vec) = &mut calendar;
            if vec.is_empty() {
                return Ok(());
            }

            let task_index: usize = get_number("Enter parent task index: ")?;

            let parent_task: &mut Task;
            match vec.get_mut(task_index) {
                Some(value) => parent_task = value,
                None => {return Err("Error, invalid Task index".to_string());},
            }

            let subtask = create_subtask()?;
            parent_task.push(subtask);
            calendar.save()
        }

        None => Err("Error, missing arguments after new".to_string()),

        _ => Err("Error, unrecognized arguments after new".to_string()),
    }
}

fn handle_read(args: Vec<String>) -> Result<(), String>{
    match args.get(2).map(String::as_str) {
        Some("related") => {
            return Ok(());
        },

        Some("task") => {
            let calendar: Calendar = Calendar::load()?;
            calendar.print_tasks();
            let task: &Task = get_task(&calendar, "Enter task index: ")?.ok_or("You have no upcoming tasks!")?;

            let dir = format!("{}/.local/share/iptm/details_files/", std::env::var("HOME").map_err(|_| "Error, failed to access user home")?);
            fs::create_dir_all(&dir).map_err(|_| "Error: failed to create directory")?;

            Command::new("/home/ideale/.nixvim/result/bin/nvim")
                .arg(task.details_file.to_str().ok_or("Error, failed to convert related file")?)
                .status()
                .map_err(|_| "Error, failed to load editor")?;
            Ok(())
        },

        Some("subtask") => {
            let calendar: Calendar = Calendar::load()?;
            calendar.print_tasks();
            let parent_task = get_task(&calendar, "Enter parent task index: ")?.ok_or("You have no upcoming tasks!")?;
            parent_task.print_subtasks()?;
            let subtask = get_subtask(&parent_task, "Enter subtask index: ")?.ok_or("Task has no subtasks yet!")?;

            let dir = format!("{}/.local/share/iptm/details_files/", std::env::var("HOME").map_err(|_| "Error, failed to access user home")?);
            fs::create_dir_all(&dir).map_err(|_| "Error: failed to create directory")?;

            Command::new("/home/ideale/.nixvim/result/bin/nvim")
                .arg(subtask.details_file.to_str().ok_or("Error, failed to convert related file")?)
                .status()
                .map_err(|_| "Error, failed to load editor")?;

            Ok(())
        },

        None => Err("Error, missing arguments after new".to_string()),

        _ => Err("Error, unrecognized arguments after new".to_string()),
    }
}

