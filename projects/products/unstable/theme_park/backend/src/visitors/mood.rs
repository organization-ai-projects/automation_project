#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Visitor mood (0–100). Higher is happier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Mood(pub i32);

impl Mood {
    pub const MAX: i32 = 100;
    pub const MIN: i32 = 0;
    pub const INITIAL: i32 = 70;

    pub fn new(v: i32) -> Self {
        Self(v.clamp(Self::MIN, Self::MAX))
    }

    pub fn value(self) -> i32 {
        self.0
    }

    pub fn adjust(self, delta: i32) -> Self {
        Self::new(self.0 + delta)
    }
}
