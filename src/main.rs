use std::env;

fn main() {
    // handle program arguments
    let args: Vec<String> = env::args().collect();

    if let Some(first_arg) = args.get(1) {
        match first_arg.as_str() {
            "hello" => println!("Hello world!"),
            _ => eprintln!("Error, unrecognized arguments"),
        }
    } else {
        eprintln!("Error, missing arguments");
    }
}

