// projects/products/unstable/platform_versioning/backend/src/issues/issue_id.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;

/// An opaque issue identifier: alphanumeric slug up to 64 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IssueId(String);

impl IssueId {
    /// Maximum length in characters.
    pub const MAX_LEN: usize = 64;

    /// Returns the string slice of this identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '-' || c == '_'
    }
}

impl fmt::Display for IssueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for IssueId {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(PvError::InvalidId("IssueId must not be empty".to_string()));
        }
        if s.len() > Self::MAX_LEN {
            return Err(PvError::InvalidId(format!(
                "IssueId must be at most {} characters",
                Self::MAX_LEN
            )));
        }
        if !s.chars().all(Self::is_valid_char) {
            return Err(PvError::InvalidId(
                "IssueId must contain only alphanumerics, hyphens, or underscores".to_string(),
            ));
        }
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        let id: IssueId = "issue-42".parse().unwrap();
        assert_eq!(id.as_str(), "issue-42");
    }

    #[test]
    fn parse_empty_rejected() {
        assert!("".parse::<IssueId>().is_err());
    }

    #[test]
    fn parse_too_long_rejected() {
        let s = "a".repeat(65);
        assert!(s.parse::<IssueId>().is_err());
    }

    #[test]
    fn parse_invalid_chars_rejected() {
        assert!("my issue".parse::<IssueId>().is_err());
        assert!("my/issue".parse::<IssueId>().is_err());
    }
}
