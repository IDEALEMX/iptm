use regex::Regex;
use chrono::{NaiveDate, Duration, Datelike, Local};

use std::str::FromStr;
use std::ops::Add;

pub trait NaiveDateExt {
    fn from_str(input: &str) -> Option<NaiveDate>;
    fn days_from_today(&self) -> i64;
}

impl NaiveDateExt for NaiveDate{
    fn from_str(input: &str) -> Option<NaiveDate> {
        let date_regex = Regex::new(r"(?x) # allows comments
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # day
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # month
        (\d{2,4}|\.|\+\d{1,2}) #year")
            .unwrap();
        if let Some(captured_date) = date_regex.captures(input) {
            let current_date = chrono::Local::now();
            let day: i64 = parse_date_element(&captured_date[1], current_date.day() as i64);
            let month: u32 = parse_date_element(&captured_date[2], current_date.month() as u32);
            let year: i32 = parse_date_element(&captured_date[3], current_date.year() as i32);
            let parsed_year: i32;

            if year < 1000 {
                parsed_year = year + 2000;
            } else {
                parsed_year = year;
            }

            // validate string -- todo

            let final_date: NaiveDate = normalize_date(parsed_year, month, day)?;
            return Some(final_date);

        } else {
            eprintln!("Error, invalid date format");
            return None;
        }
    }

    fn days_from_today(&self) -> i64 {
        let today = Local::now().date_naive();
        (today - *self).num_days()
    }

}

pub fn normalize_date(year: i32, mut month: u32, day: i64) -> Option<NaiveDate> {
    // Start from the first of the month
    month = month.max(1).min(12); // clamp month to 1–12
    let base = NaiveDate::from_ymd_opt(year, month, 1)?;
    // Add (day - 1) days to move to the correct day, with overflow
    let normalized = base.checked_add_signed(Duration::days(day - 1))?;
    Some(normalized)
}

pub fn parse_date_element<T>(element_string: &str, current: T) -> T 
where
    T: Add<Output = T> + FromStr, <T as FromStr>::Err: std::fmt::Debug,
{
    let first_char: char = element_string.chars().next().unwrap();
    let char_rest: &str = &element_string.to_string()[1..];

    if first_char == '.' {
        return current
    }

    if first_char == '+' {
        let change = char_rest.parse().unwrap();
        return current + change
    }

    return element_string.parse().unwrap()
}
