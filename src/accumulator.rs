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
