#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Pricing policy for the park.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub admission_fee: u32,
}

impl Pricing {
    pub fn new(admission_fee: u32) -> Self {
        Self { admission_fee }
    }
}

impl Default for Pricing {
    fn default() -> Self {
        Self { admission_fee: 20 }
    }
}
