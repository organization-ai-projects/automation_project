use crate::model::FuzzInput;
use crate::rng::SeededRng;

pub(crate) struct InputGenerator {
    rng: SeededRng,
    index: u64,
}

impl InputGenerator {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            rng: SeededRng::from_seed(seed),
            index: 0,
        }
    }

    pub(crate) fn next(&mut self) -> FuzzInput {
        let len_raw = self.rng.next_u64();
        let len = ((len_raw % 256) + 1) as usize;
        let data = self.rng.next_bytes(len);
        let index = self.index;
        self.index += 1;
        FuzzInput { data, index }
    }
}
