use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::process;

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
