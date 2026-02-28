#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReport {
    pub kind: String,
    pub results: Vec<String>,
}
