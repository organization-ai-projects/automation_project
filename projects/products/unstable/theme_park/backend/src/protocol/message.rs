#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Envelope wrapping every IPC message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub payload: Value,
}
