#![allow(dead_code)]
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngDraw {
    pub tick: Tick,
    pub context: String,
    pub value: u64,
}

impl RngDraw {
    pub fn new(tick: Tick, context: impl Into<String>, value: u64) -> Self {
        Self {
            tick,
            context: context.into(),
            value,
        }
    }
}
