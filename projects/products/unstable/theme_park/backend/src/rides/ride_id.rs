#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Unique identifier for a ride.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RideId(pub u32);

impl RideId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for RideId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ride({})", self.0)
    }
}
