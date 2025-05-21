pub mod date;
pub mod task;
pub mod id;
pub mod calendar;

use crate::date::NaiveDateExt; 

use calendar::Calendar;
use chrono::NaiveDate;
use task::Task;
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
                let new_task: Task = Task::new(&calendar, name, due_date, None, false).unwrap();
                calendar.add(new_task);
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

