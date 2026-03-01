// projects/products/unstable/hospital_tycoon/backend/src/reputation/reputation.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reputation {
    pub score: u32,
    pub max_score: u32,
}

impl Reputation {
    pub fn new(initial_score: u32) -> Self {
        Self {
            score: initial_score,
            max_score: 100,
        }
    }

    pub fn increase(&mut self, amount: u32) {
        self.score = (self.score + amount).min(self.max_score);
    }

    pub fn decrease(&mut self, amount: u32) {
        self.score = self.score.saturating_sub(amount);
    }
}
