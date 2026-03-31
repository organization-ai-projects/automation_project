use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChecksumValue(pub String);

impl ChecksumValue {
    pub fn new(hex_value: String) -> Self {
        Self(hex_value)
    }
}

impl std::fmt::Display for ChecksumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
