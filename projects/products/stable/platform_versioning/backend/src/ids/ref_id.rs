// projects/products/stable/platform_versioning/backend/src/ids/ref_id.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;

/// Unique identifier for a ref (branch or tag) within a repository.
///
/// Format: alphanumerics, hyphens, underscores, and forward slashes.
/// Leading/trailing slashes are rejected. Double slashes are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RefId(String);

impl RefId {
    /// Maximum length in characters.
    pub const MAX_LEN: usize = 128;

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid(s: &str) -> bool {
        if s.is_empty() || s.len() > Self::MAX_LEN {
            return false;
        }
        if s.starts_with('/') || s.ends_with('/') {
            return false;
        }
        if s.contains("//") {
            return false;
        }
        s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/')
    }
}

impl fmt::Display for RefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for RefId {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !Self::is_valid(s) {
            return Err(PvError::InvalidId(format!(
                "RefId '{}' is invalid: must be non-empty, ≤{} chars, \
                 alphanumeric/hyphen/underscore/slash, no leading/trailing/double slashes",
                s,
                Self::MAX_LEN
            )));
        }
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        let id: RefId = "heads/main".parse().unwrap();
        assert_eq!(id.as_str(), "heads/main");
    }

    #[test]
    fn parse_empty() {
        assert!("".parse::<RefId>().is_err());
    }

    #[test]
    fn parse_leading_slash() {
        assert!("/heads/main".parse::<RefId>().is_err());
    }

    #[test]
    fn parse_double_slash() {
        assert!("heads//main".parse::<RefId>().is_err());
    }

    #[test]
    fn parse_spaces() {
        assert!("heads/my branch".parse::<RefId>().is_err());
    }
}
