pub mod date;
pub mod task;
pub mod subtask;
pub mod day;
pub mod calendar;

use crate::date::NaiveDateExt; 

use calendar::Calendar;
use chrono::NaiveDate;
use subtask::Subtask;
use task::Task;
use day::Day;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        match first_arg.as_str() {
            "hello" => println!("Hello world!"),
            "date" => {
                let input: String = get_input("enter date: ");
                println!("{}", NaiveDate::from_str(input.as_str()).unwrap());
            },
            "new" => {
                let mut calendar: Calendar = Calendar::load().expect("Failed to open calendar file, check integrity");
                let due_date: NaiveDate = NaiveDate::from_str("+1/./.").unwrap();
                let name: &str = "test task";
                let mut new_task: Task = Task::new(name, due_date).unwrap();

                let new_subtask: Subtask = Subtask::new("test_subtask", Day(5)).unwrap();
                new_task.push(new_subtask);

                calendar.push(new_task);
                calendar.print();
                match calendar.save() {
                    Ok(_) => (),
                    Err(err) => panic!("{err}"),
                };
            },

            "list" => {
                let calendar: Calendar = Calendar::load().expect("Failed to open calendar file, check integrity");
                calendar.print();
                match calendar.save() {
                    Ok(_) => (),
                    Err(err) => panic!("{err}"),
                };
            }
            _ => eprintln!("Error, unrecognized arguments"),
        }
    } else {
        eprintln!("Error, missing arguments");
    }
}

fn get_input(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error, failed to read user input");
    return user_input.trim_end().to_string()
}

