const B32A: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const B64A: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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
