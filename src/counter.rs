pub struct Counter {
    count: usize,
    limit: usize,
}

impl Counter {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn build(chunk_size: usize) -> Counter {
        Counter {
            count: 0,
            limit: chunk_size,
        }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn check_reset(&mut self) -> bool {
        if self.count >= self.limit {
            self.count = 0;
        }
        self.count == 0
    }
}
