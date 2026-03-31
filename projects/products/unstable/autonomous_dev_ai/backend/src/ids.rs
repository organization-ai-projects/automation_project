//! projects/products/unstable/autonomous_dev_ai/src/ids.rs
use std::fmt;

use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IssueNumber(u64);

impl IssueNumber {
    pub fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for IssueNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrNumber(u64);

impl PrNumber {
    pub fn new(value: u64) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }
}

impl fmt::Display for PrNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRef {
    None,
    Issue(IssueNumber),
}

impl ParentRef {
    pub fn parse(input: &str) -> Option<Self> {
        let value = input.trim();
        if value.eq_ignore_ascii_case("none") {
            return Some(Self::None);
        }
        let raw_num = value.strip_prefix('#').unwrap_or(value);
        let num = raw_num.parse::<u64>().ok()?;
        Some(Self::Issue(IssueNumber::new(num)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(ProtocolId);

impl RunId {
    pub fn new() -> Self {
        let protocol_id = ProtocolId::generate();
        Self(protocol_id)
    }
}

impl fmt::Display for RunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for RunId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActorId(ProtocolId);

impl ActorId {
    pub fn new() -> Self {
        let protocol_id = ProtocolId::generate();
        Self(protocol_id)
    }
}

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ActorId {
    fn default() -> Self {
        Self::new()
    }
}
