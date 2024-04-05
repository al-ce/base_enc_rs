use std::io::{self, BufWriter, Write};
use std::process;

mod accumulator;
mod args;
mod base;
mod counter;
mod utils;

const WRAP_LIMIT: usize = 76;

fn main() -> io::Result<()> {
    // Wrap stdout to keep it from flushing until end of program
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // ----------
    // Parse args
    // ----------
    let args = args::ParsedArgs::get().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let filename = args.filename;

    let base_arg = args.base;
    let base = base::Base::build(base_arg).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let base_alphabet = base.alphabet;

    // ----------
    // Initialize structs
    // ----------
    let mut ac = accumulator::Accumulator::build();
    let mut byte_counter = counter::Counter::build(base.chunk_size);
    let mut wrap_counter = counter::Counter::build(WRAP_LIMIT);

    // ----------
    // Begin encoding
    // ----------
    let lines = utils::get_lines(filename);

    for line in lines {
        let line = line.unwrap();
        for byte in line.bytes() {
            byte_counter.increment();
            byte_counter.check_reset();

            let byte: u16 = (byte).into();
            ac.accumulate(byte);
            ac.bits += 8;

            while ac.bits() >= base.log {
                ac.bits -= base.log;

                let idx: usize = (ac.byteval() >> ac.bits()).into();
                write!(stdout, "{}", &base_alphabet[idx..idx + 1])?;

                ac.mask_off_bits();

                wrap_counter.increment();
                if wrap_counter.check_reset() {
                    writeln!(stdout)?;
                }
            }

            assert!((0..base.log).contains(&ac.bits()));
        }
    }

    // ----------
    // Handle bits leftover in the accumulator
    // ----------
    if byte_counter.count() != 0 {
        let idx: usize = (ac.byteval() << (base.log - ac.bits())).into();
        write!(stdout, "{}", &base_alphabet[idx..idx + 1])?;

        let count = (base.chunk_size - byte_counter.count()) * 8 / base.log;
        for _ in 0..count {
            write!(stdout, "=")?;
        }

        if wrap_counter.count() == 0 {
            writeln!(stdout)?;
        }
    }

    // ----------
    // Flush and print final line break
    // ----------
    writeln!(stdout)?;

    stdout.flush()?;

    Ok(())
}
