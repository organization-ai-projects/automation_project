#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Turn(pub u64);

impl Turn {
    pub fn zero() -> Self { Self(0) }
    pub fn next(self) -> Self { Self(self.0 + 1) }
}
