#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackId(pub String);

impl PackId {
    pub fn new(v: impl Into<String>) -> Self { Self(v.into()) }
}
