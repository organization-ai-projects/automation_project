pub(crate) struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub(crate) fn from_seed(seed: u64) -> Self {
        let state = if seed == 0 { 0x9E3779B97F4A7C15 } else { seed };
        Self { state }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub(crate) fn next_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(len);
        let mut remaining = len;
        while remaining > 0 {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            let take = remaining.min(8);
            result.extend_from_slice(&bytes[..take]);
            remaining -= take;
        }
        result
    }
}
