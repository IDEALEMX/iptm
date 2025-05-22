use regex::Regex;
use chrono::{NaiveDate, Duration, Datelike, Local};

use std::str::FromStr;
use std::ops::Add;

pub trait NaiveDateExt {
    fn from_str(input: &str) -> Result<NaiveDate, String>;
    fn days_from_today(&self) -> i64;
}

impl NaiveDateExt for NaiveDate{
    fn from_str(input: &str) -> Result<NaiveDate, String> {
        let date_regex = Regex::new(r"(?x) # allows comments
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # day
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # month
        (\d{2,4}|\.|\+\d{1,2}) #year").map_err(|_| "Error, non valid date syntax")?;

        let captured_date = date_regex.captures(input).ok_or("Error, invalid date format")?;
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

        return normalize_date(parsed_year, month, day);

    }

    fn days_from_today(&self) -> i64 {
        let today = Local::now().date_naive();
        (*self - today).num_days()
    }

}

pub fn normalize_date(year: i32, mut month: u32, day: i64) -> Result<NaiveDate, String> {
     month = month.clamp(1, 12);
    let base = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or("Error: normalization error #1")?;

    base.checked_add_signed(Duration::days(day - 1))
        .ok_or("Error: normalization error #2".to_string())
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
