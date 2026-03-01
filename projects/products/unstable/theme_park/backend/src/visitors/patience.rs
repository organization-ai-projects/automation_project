#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Visitor patience (ticks remaining before they give up waiting).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Patience(pub u32);

impl Patience {
    pub const INITIAL: u32 = 30;

    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn is_exhausted(self) -> bool {
        self.0 == 0
    }

    pub fn decay(self) -> Self {
        Self(self.0.saturating_sub(1))
    }
}
