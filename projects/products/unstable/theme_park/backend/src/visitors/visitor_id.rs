#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Unique identifier for a visitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VisitorId(pub u32);

impl VisitorId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for VisitorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "visitor({})", self.0)
    }
}
