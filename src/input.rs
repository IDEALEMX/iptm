use crate::{calendar, date::NaiveDateExt}; 
use crate::subtask::Subtask;
use crate::task::Task;
use crate::day::Day;
use crate::calendar::Calendar;

use std::{io::{self, Write}, usize};
use chrono::NaiveDate;
use regex::Regex;

fn get_input(message: &str) -> Result<String, String> {
    print!("{message}");
    io::stdout().flush().map_err(|_| "Error, user input failed")?;
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).map_err(|_| "Error, failed to read user input")?;
    Ok(user_input.trim_end().to_string())
}

pub fn get_number(message: &str) -> Result<usize, String> {
    let number_string = get_input(message)?;
    let date_regex = Regex::new(r"(?x) # allows comments
        (\d*) #year")
        .map_err(|_| "Error, non valid date syntax")?;
    let captured_number = date_regex.captures(number_string.as_str()).ok_or("Error, invalid date format")?;
    let number: usize = captured_number[1]
        .parse()
        .map_err(|_| "failed to parse error")?;
    Ok(number)
}

pub fn get_task<'a>(calendar: &'a Calendar, message: &'a str) -> Result<Option<&'a Task>, String> {
    let Calendar(vec) = calendar;
    if vec.is_empty() {
        return Ok(None);
    }
    let task_index: usize = get_number(message)?;
    if let Some(task) = vec.get(task_index) {
        return Ok(Some(task));
    } else {
        return  Err("Error, non valid index".to_string());
    }

}

pub fn get_date(message: &str) -> Result<NaiveDate, String>{
    let date_input: String = get_input(message)?;
    return NaiveDate::from_str(date_input.as_str());
}

pub fn get_days(message: &str) -> Result<Day, String>{
    let date_input: String = get_input(message)?;
    return Day::from_str(date_input.as_str());
}

pub fn create_task() -> Result<Task, String> {
    let name: String = get_input("Enter task name: ")?;
    let due_date: NaiveDate = get_date("Enter due date: ")?;
    Task::new(name.as_str(), due_date)
}

pub fn create_subtask() -> Result<Subtask, String> {
    let name: String = get_input("Enter subtask name: ")?;
    let days: Day = get_days("Enter duration relative date: ")?;
    Subtask::new(name.as_str(), days)
}
