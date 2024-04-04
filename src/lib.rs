use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::process;

pub fn parse_args(args: &[String]) -> Result<String, &'static str> {
    match args.len() {
        1 => {
            eprintln!("No arguments");
            Ok("-".to_string())
        }
        2 => {
            eprintln!("Processing one argument");
            Ok(env::args().nth(1).unwrap())
        }
        _ => Err("Too many arguments"),
    }
}

fn open_file(filename: String) -> File {
    File::open(filename).unwrap_or_else(|err| {
        eprintln!("Could not open file : {err}");
        process::exit(1);
    })
}

pub fn get_lines(filename: String) -> Box<dyn Iterator<Item = io::Result<String>>> {
    let lines: Box<dyn Iterator<Item = io::Result<String>>> = if filename == "-" {
        Box::new(io::BufReader::new(io::stdin()).lines())
    } else {
        let f = open_file(filename);
        Box::new(io::BufReader::new(f).lines())
    };
    lines
}


pub struct Counter {
    pub count: usize,
    limit: usize,
}

impl Counter {
    pub fn init(limit: usize) -> Counter {
        Counter { count: 0, limit }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn check_reset(&mut self) -> bool {
        if self.count >= self.limit {
            self.count = 0;
        }
        self.count == 0
    }
}
