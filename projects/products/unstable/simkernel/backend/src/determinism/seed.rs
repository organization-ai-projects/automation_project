#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed(pub u64);

impl Seed {
    pub fn new(v: u64) -> Self { Self(v) }
    pub fn value(self) -> u64 { self.0 }
}
