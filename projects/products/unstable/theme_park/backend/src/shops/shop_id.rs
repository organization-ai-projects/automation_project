#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Unique identifier for a shop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ShopId(pub u32);

impl ShopId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for ShopId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shop({})", self.0)
    }
}
