use std::env;
use std::fs::File;
use std::io::{self};
use std::process;

struct WrapCounter {
    count: usize,
    limit: usize,
}

impl WrapCounter {
    fn init(&self, limit: usize) {
        WrapCounter{count: 0, limit}
    }
    fn need_wrap(&self) -> bool {
        self.count += 1;
        if self.count >= self.limit {
            self.count = 0;
            return true;
        }
        false
    }
}

fn main() -> io::Result<()> {
    let b64a = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let args: Vec<String> = env::args().collect();

    let filename = base64_rs::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    eprintln!("filename: {:?}", filename);

    let lines: Box<dyn Iterator<Item = io::Result<String>>> = if filename == "-" {
        Box::new(io::BufReader::new(io::stdin()).lines())
    } else {
        let f = File::open(filename)?;
        Box::new(io::BufReader::new(f).lines())
    };

    eprintln!("Beginning base 64 encoding");

    let mut ac: u16 = 0;
    let mut ac_bits = 0;
    let mut byte_counter = 0;
    // let mut wrap_counter = 0;
    let wrap_counter = WrapCounter.init();

    for line in lines {
        let line = line.unwrap();
        for byte in line.bytes() {
            byte_counter += 1;
            if byte_counter == 3 {
                byte_counter = 0;
            }

            let byte: u16 = (byte).into();
            ac <<= 8;
            ac |= byte;
            ac_bits += 8;

            while ac_bits >= 6 {
                ac_bits -= 6;
                let idx: usize = (ac >> ac_bits).into();
                ac &= (1 << ac_bits) - 1;

                print!("{}", &b64a[idx..idx + 1]);

                if wrap_counter.inc() {
                    print!("\n")
                }
                // wrap_counter += 1;
                // if wrap_counter >= 76 {
                //     wrap_counter = 0;
                //     print!("\n");
                // }
            }

            assert!((0..6).contains(&ac_bits));
        }
    }

    if byte_counter != 0 {
        let idx: usize = (ac << (6 - ac_bits)).into();
        print!("{}", &b64a[idx..idx + 1]);

        let count = (3 - byte_counter) * 8 / 6;
        for _ in 0..count {
            print!("=");
        }

        if wrap_counter != 0 {
            print!("\n");
        }
    }

    Ok(())
}
