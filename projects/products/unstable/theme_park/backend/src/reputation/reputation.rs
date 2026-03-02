#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Park reputation score (0–100).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reputation {
    pub score: i32,
}

impl Reputation {
    pub const MAX: i32 = 100;
    pub const MIN: i32 = 0;
    pub const INITIAL: i32 = 50;

    pub fn new(initial: i32) -> Self {
        Self {
            score: initial.clamp(Self::MIN, Self::MAX),
        }
    }

    pub fn set(&mut self, v: i32) {
        self.score = v.clamp(Self::MIN, Self::MAX);
    }
}
