
use std::fmt::{self, Display, Formatter};
use regex::Regex;
use chrono::Datelike;

pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn from_str(input: &str) -> Option<Date> {
        let date_regex = Regex::new(r"(?x) # allows comments
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # day
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # month
        (\d{2,4}|\.|\+\d{1,2}) #year")
            .unwrap();
        if let Some(captured_date) = date_regex.captures(input) {
            let current_date = chrono::Utc::now();
            let day: u8 = parse_date_element(&captured_date[1], current_date.day() as u8);
            let month: u8 = parse_date_element(&captured_date[2], current_date.month() as u8);
            let year: u16 = parse_date_element(&captured_date[3], current_date.year() as u16);
            let parsed_year: u16;

            if year < 1000 {
                parsed_year = year + 2000;
            } else {
                parsed_year = year;
            }

            // validate string -- todo

            let final_date = Date{day, month, year: parsed_year};
            return Some(final_date);

        } else {
            eprintln!("Error, invalid date format");
            return None;
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}-{:02}-{:04}",self.day , self.month, self.year)
    }
}

use std::str::FromStr;
use std::ops::Add;

fn parse_date_element<T>(element_string: &str, current: T) -> T 
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
