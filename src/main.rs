use std::env;
use std::io::{self, BufWriter, Write};
use std::process;

const BASE: usize = 64; // could also work for base 32
const BASE_EXP: usize = base64_rs::log(BASE); // e.g. base16: 4, base32: 5, base64: 6
const CHUNK_SIZE: usize = base64_rs::get_byte_chunk_size(BASE_EXP);
const WRAP_LIMIT: usize = 76;

fn main() -> io::Result<()> {

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let bxxa = base64_rs::BaseAlphabet::build(&BASE).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let args: Vec<String> = env::args().collect();

    let filename = base64_rs::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    eprintln!("filename: {:?}", filename);
    let lines = base64_rs::get_lines(filename);

    eprintln!("Beginning base {} encoding", BASE);

    let mut ac = base64_rs::Accumulator::build();
    let mut byte_counter = base64_rs::Counter::build(CHUNK_SIZE);
    let mut wrap_counter = base64_rs::Counter::build(WRAP_LIMIT);

    for line in lines {
        let line = line.unwrap();
        for byte in line.bytes() {
            byte_counter.increment();
            byte_counter.check_reset();

            let byte: u16 = (byte).into();
            ac.accumulate(byte);
            ac.bits += 8;

            while ac.bits() >= BASE_EXP {
                ac.bits -= BASE_EXP;

                let idx: usize = (ac.byteval() >> ac.bits()).into();
                write!(stdout, "{}", &bxxa[idx..idx + 1])?;

                ac.mask_off_bits();

                wrap_counter.increment();
                if wrap_counter.check_reset() {
                    writeln!(stdout)?;
                }
            }

            assert!((0..BASE_EXP).contains(&ac.bits()));
        }
    }

    if byte_counter.count() != 0 {
        let idx: usize = (ac.byteval() << (BASE_EXP - ac.bits())).into();
        print!("{}", &bxxa[idx..idx + 1]);

        let count = (CHUNK_SIZE - byte_counter.count()) * 8 / BASE_EXP;
        for _ in 0..count {
            print!("=");
        }
    }

    writeln!(stdout)?;

    stdout.flush()?;

    Ok(())
}
