use std::env;
use std::io::{self};
use std::process;

const WRAP_LIMIT: usize = 76;
const BYTE_LIMIT: usize = 3;
const BASE_MULTIPLIER: usize = 6; // e.g. base16: 4, base32: 5, base64: 6

fn main() -> io::Result<()> {
    let b64a = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let args: Vec<String> = env::args().collect();

    let filename = base64_rs::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    eprintln!("filename: {:?}", filename);
    let lines = base64_rs::get_lines(filename);

    eprintln!("Beginning base 64 encoding");

    let mut ac = base64_rs::Accumulator::build();
    let mut byte_counter = base64_rs::Counter::build(BYTE_LIMIT);
    let mut wrap_counter = base64_rs::Counter::build(WRAP_LIMIT);

    for line in lines {
        let line = line.unwrap();
        for byte in line.bytes() {
            byte_counter.increment();
            byte_counter.check_reset();

            let byte: u16 = (byte).into();
            ac.accumulate(byte);
            ac.bits += 8;

            while ac.bits() >= BASE_MULTIPLIER {
                ac.bits -= BASE_MULTIPLIER;
                let idx: usize = (ac.byteval() >> ac.bits()).into();
                ac.mask_off_bits();

                print!("{}", &b64a[idx..idx + 1]);

                wrap_counter.increment();
                if wrap_counter.check_reset() {
                    print!("\n")
                }
            }

            assert!((0..6).contains(&ac.bits()));
        }
    }

    if byte_counter.count != 0 {
        let idx: usize = (ac.byteval() << (6 - ac.bits())).into();
        print!("{}", &b64a[idx..idx + 1]);

        let count = (3 - byte_counter.count) * 8 / 6;
        for _ in 0..count {
            print!("=");
        }

        if wrap_counter.count != 0 {
            print!("\n");
        }
    }

    Ok(())
}
