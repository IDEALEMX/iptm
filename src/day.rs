use crate::date::{normalize_date, parse_date_element};
use crate::date::NaiveDateExt; 

use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Datelike, Local};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct Day(pub i64);

impl Day {
    pub fn from_str(input: &str) -> Result<Day, String> {
        let date_regex = Regex::new(r"(?x) # allows comments
        (\.|\+\d{1,2})[\-\/] # day
        (\.|\+\d{1,2})[\-\/] # month
        (\.|\+\d{1,2}) #year")
            .map_err(|_| "Error, non valid date syntax")?;
        let captured_date = date_regex.captures(input).ok_or("Error, invalid date format")?;
        let current_date = chrono::Local::now();
        let day: i64 = parse_date_element(&captured_date[1], current_date.day() as i64);
        let month: u32 = parse_date_element(&captured_date[2], current_date.month() as u32);
        let year: i32 = parse_date_element(&captured_date[3], current_date.year() as i32);
        let final_date: NaiveDate = normalize_date(year, month, day)?;
        let day_difference: i64 = final_date.days_from_today();

        Ok(Day(day_difference))
    }
}
