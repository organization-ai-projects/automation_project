// projects/products/unstable/platform_versioning/backend/src/ids/repo_id.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;

/// A repository identifier: URL-safe alphanumeric slug up to 64 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RepoId(String);

impl RepoId {
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

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for RepoId {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(PvError::InvalidId("RepoId must not be empty".to_string()));
        }
        if s.len() > Self::MAX_LEN {
            return Err(PvError::InvalidId(format!(
                "RepoId must be at most {} characters",
                Self::MAX_LEN
            )));
        }
        if !s.chars().all(Self::is_valid_char) {
            return Err(PvError::InvalidId(
                "RepoId must contain only alphanumerics, hyphens, or underscores".to_string(),
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
        let id: RepoId = "my-repo_1".parse().unwrap();
        assert_eq!(id.as_str(), "my-repo_1");
    }

    #[test]
    fn parse_empty() {
        assert!("".parse::<RepoId>().is_err());
    }

    #[test]
    fn parse_too_long() {
        let s = "a".repeat(65);
        assert!(s.parse::<RepoId>().is_err());
    }

    #[test]
    fn parse_invalid_chars() {
        assert!("my repo".parse::<RepoId>().is_err());
        assert!("my/repo".parse::<RepoId>().is_err());
    }
}
