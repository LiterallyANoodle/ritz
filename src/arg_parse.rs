// Submodule for parsing command line arguments
use std::env;

pub fn parse() {
    let args: Vec<String> = env::args().collect();

    match args.len() {

        // no args
        1 => {
            println!("Called arg_parse.parse() with no arguments.");
        }

        // 1 arg
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is 42"),
                _ => println!("This is not 42"),
            }
        }

        // default case
        _ => {
            println!("Match failed.");
        }

    }
}