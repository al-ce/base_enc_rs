const B32A: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const B64A: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub struct Base {
    pub base: usize,
    pub alphabet: String,
    pub log: usize,
    pub chunk_size: usize,
}

impl Base {
    pub fn build(base: usize) -> Result<Base, &'static str> {
        let log = get_base_log(base);
        let chunk_size = get_byte_chunk_size(log);
        let alphabet = match base {
            32 => B32A.to_string(),
            64 => B64A.to_string(),
            _ => return Err("Invalid base"),
        };
        Ok(Base {
            alphabet,
            base,
            log,
            chunk_size,
        })
    }
}

// https://github.com/TheAlgorithms/Rust/blob/1c6c38d12be0ab45dd365804d478c011dddec325/src/math/lcm_of_n_numbers.rs
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub const fn get_base_log(base: usize) -> usize {
    let mut base = base;
    let mut log = 1;
    while base != 2 {
        log += 1;
        base /= 2;
    }
    log
}

pub fn get_byte_chunk_size(base_exp: usize) -> usize {
    base_exp / gcd_of_two_numbers(base_exp, 8)
}
