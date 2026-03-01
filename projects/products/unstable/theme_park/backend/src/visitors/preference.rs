#![allow(dead_code)]
use crate::rides::ride_kind::RideKind;
use serde::{Deserialize, Serialize};

/// Visitor's ride preferences (ordered list of preferred ride kinds).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
    pub preferred_kinds: Vec<RideKind>,
}

impl Preference {
    pub fn new(preferred_kinds: Vec<RideKind>) -> Self {
        Self { preferred_kinds }
    }

    pub fn score(&self, kind: &RideKind) -> i32 {
        self.preferred_kinds
            .iter()
            .position(|k| k == kind)
            .map(|pos| (self.preferred_kinds.len() - pos) as i32)
            .unwrap_or(0)
    }
}
