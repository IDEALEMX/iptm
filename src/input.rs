use std::io::{self, Write};
use chrono::NaiveDate;
use crate::date::NaiveDateExt; 
use crate::task::Task;

fn get_input(message: &str) -> Result<String, String> {
    print!("{message}");
    io::stdout().flush().map_err(|_| "Error, user input failed")?;
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).map_err(|_| "Error, failed to read user input");
    Ok(user_input.trim_end().to_string())
}

pub fn get_date() -> Result<NaiveDate, String>{
    let date_input: String = get_input("Enter date: ")?;
    return NaiveDate::from_str(date_input.as_str());
}

pub fn get_task() -> Result<Task, String> {
    let name: String = get_input("Enter task name: ")?;
    let due_date: NaiveDate = get_date()?;
    Task::new(name.as_str(), due_date)
}
