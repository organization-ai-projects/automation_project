#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Explicit seed holder for the simulation RNG.
/// All randomness must be derived from this seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed(pub u64);

impl Seed {
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    pub fn value(self) -> u64 {
        self.0
    }

    /// Derive a sub-seed for a domain by mixing with a domain tag.
    pub fn derive(self, tag: u64) -> u64 {
        self.0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(tag)
    }
}
