pub mod date;

use crate::date::Date;
use std::env;
use std::io::{self, Write};

fn main() {
    // handle program arguments
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        match first_arg.as_str() {
            "hello" => println!("Hello world!"),
            "date" => {
                let input: String = get_input("enter date: ");
                println!("{}", Date::from_str(input.as_str()).unwrap());
            },
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

