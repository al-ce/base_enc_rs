use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::process;

pub const fn log(base: usize) -> usize {
    let mut base = base;
    let mut log = 1;
    while base != 2 {
        log += 1;
        base /= 2;
    }
    log
}

pub const fn get_byte_chunk_size(base_exp: usize) -> usize {
    base_exp / gcd_of_two_numbers(base_exp, 8)
}

// https://github.com/TheAlgorithms/Rust/blob/1c6c38d12be0ab45dd365804d478c011dddec325/src/math/lcm_of_n_numbers.rs
const fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

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
