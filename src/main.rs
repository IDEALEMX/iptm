use std::env;
use std::io::{self, Write};
use chrono::Datelike;

use regex::Regex;

fn main() {
    // handle program arguments
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        match first_arg.as_str() {
            "hello" => println!("Hello world!"),
            "parse" => get_date(get_input("enter date: ").as_str()),
            _ => eprintln!("Error, unrecognized arguments"),
        }
    } else {
        eprintln!("Error, missing arguments");
    }
}

struct Date {
    day: u8,
    month: u8,
    year: u16,
}

fn parse_date(input: &str) {
    let date_regex = Regex::new(r"(?x) # allows comments
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # day
        (\d{1,2}|\.|\+\d{1,2})[\-\/] # month
        (\d{2,4}|\.|\+\d{1,2}) #year")
        .unwrap();
    if let Some(captured_date) = date_regex.captures(input) {
        println!("day: {}, month: {}, year: {}", &captured_date[1], &captured_date[2], &captured_date[3]);
    } else {
        eprintln!("Error, invalid date format");
    }
}

fn parse_date_element(element_string: &str, current: i32) -> i32 {
    let first_char: char = element_string.chars().next().unwrap();
    let char_rest: &str = &element_string.to_string()[1..];
    if first_char == '.' {
        return current
    }

    if (first_char == '+') {
        return first_char + char_rest.
    }

    return current
}

fn get_input(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error, failed to read user input");
    return user_input.trim_end().to_string()
}

