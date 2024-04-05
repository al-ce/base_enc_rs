use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::process;

const B32A: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const B64A: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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

pub struct BaseAlphabet {}
impl BaseAlphabet {
    pub fn build(base: &usize) -> Result<&str, &'static str> {
        match base {
            32 => Ok(B32A),
            64 => Ok(B64A),
            _ => Err("Base not implemented"),
        }
    }
}

pub struct Counter {
    count: usize,
    chunk_size: usize,
}

impl Counter {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn build(chunk_size: usize) -> Counter {
        Counter {
            count: 0,
            chunk_size,
        }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn need_wrap(&mut self, wrap_limit: usize) -> bool {
        self.count >= wrap_limit
    }

    pub fn check_reset(&mut self) {
        if self.count >= self.chunk_size {
            self.count = 0;
        }
    }
}

pub struct Accumulator {
    byteval: u16,
    pub bits: usize,
}

impl Accumulator {
    pub fn bits(&self) -> usize {
        self.bits
    }
    pub fn byteval(&self) -> u16 {
        self.byteval
    }
    pub fn mask_off_bits(&mut self) {
        self.byteval &= (1 << self.bits) - 1;
    }
    pub fn build() -> Accumulator {
        Accumulator {
            byteval: 0,
            bits: 0,
        }
    }

    pub fn accumulate(&mut self, byteval: u16) {
        self.byteval <<= 8;
        self.byteval |= byteval;
    }
}
