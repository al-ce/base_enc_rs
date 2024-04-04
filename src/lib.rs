use std::env;
use std::fs::File;
use std::io::{self};

pub fn parse_args(args: &[String]) -> Result<String, &'static str> {
    match args.len() {
        1 => {
            println!("No arguments");
            Ok("-".to_string())
        }
        2 => {
            println!("Processing one argument");
            Ok(env::args().nth(1).unwrap())
        }
        _ => Err("Too many arguments"),
    }
}

pub fn get_input(filename: &String) -> Box<dyn Iterator<Item = io::Result<String>>> {
    if filename == "-" {
        Box::new(io::BufReader::new(io::stdin()).lines())
    } else {
        let f = File::open(filename);
        Box::new(io::BufReader::new(f).lines())
    }
}
