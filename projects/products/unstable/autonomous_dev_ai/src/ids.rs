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

impl std::fmt::Display for IssueNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for PrNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
