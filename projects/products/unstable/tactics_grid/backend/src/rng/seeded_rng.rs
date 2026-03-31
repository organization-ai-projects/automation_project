use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::Rng;
use super::seed::Seed;
use super::rng_draw::RngDraw;

pub struct SeededRng {
    rng: SmallRng,
    draws: Vec<RngDraw>,
}

impl SeededRng {
    pub fn new(seed: Seed) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed.0),
            draws: Vec::new(),
        }
    }

    pub fn draw_u64(&mut self, context: &str) -> u64 {
        let value: u64 = self.rng.random();
        self.draws.push(RngDraw {
            context: context.to_string(),
            value,
        });
        value
    }

    pub fn draw_range(&mut self, context: &str, min: i32, max: i32) -> i32 {
        let value: u64 = self.rng.random();
        self.draws.push(RngDraw {
            context: context.to_string(),
            value,
        });
        let range = (max - min + 1) as u64;
        min + (value % range) as i32
    }

    pub fn into_draws(self) -> Vec<RngDraw> {
        self.draws
    }
}
