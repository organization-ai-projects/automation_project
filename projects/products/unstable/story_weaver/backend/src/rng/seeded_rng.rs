use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub struct SeededRng {
    rng: SmallRng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn draw_u64(&mut self, _context: &str) -> u64 {
        self.rng.random()
    }
}
