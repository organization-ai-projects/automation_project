// projects/products/stable/platform_versioning/backend/src/refs_store/ref_name.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::refs_store::RefKind;

/// A fully-qualified ref name such as `heads/main` or `tags/v1.0`.
///
/// # Naming rules
/// - Must start with `heads/` (branch) or `tags/` (tag).
/// - The component after the prefix must be non-empty and contain only
///   alphanumerics, hyphens, underscores, or dots.
/// - Maximum total length is 128 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RefName(String);

impl RefName {
    /// Maximum total length in characters.
    pub const MAX_LEN: usize = 128;

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the [`RefKind`] encoded in this name.
    pub fn kind(&self) -> RefKind {
        if self.0.starts_with("tags/") {
            RefKind::Tag
        } else {
            RefKind::Branch
        }
    }

    /// Returns the short name (the component after the prefix).
    pub fn short_name(&self) -> &str {
        if let Some(rest) = self.0.strip_prefix("heads/") {
            rest
        } else if let Some(rest) = self.0.strip_prefix("tags/") {
            rest
        } else {
            &self.0
        }
    }

    fn is_valid_short_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.'
    }
}

impl fmt::Display for RefName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for RefName {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > Self::MAX_LEN {
            return Err(PvError::InvalidId(format!(
                "RefName must be ≤{} chars",
                Self::MAX_LEN
            )));
        }
        let short = if let Some(rest) = s.strip_prefix("heads/") {
            rest
        } else if let Some(rest) = s.strip_prefix("tags/") {
            rest
        } else {
            return Err(PvError::InvalidId(
                "RefName must start with 'heads/' or 'tags/'".to_string(),
            ));
        };
        if short.is_empty() {
            return Err(PvError::InvalidId(
                "RefName short component must be non-empty".to_string(),
            ));
        }
        if !short.chars().all(Self::is_valid_short_char) {
            return Err(PvError::InvalidId(format!(
                "RefName short component '{}' contains invalid characters",
                short
            )));
        }
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_branch() {
        let r: RefName = "heads/main".parse().unwrap();
        assert_eq!(r.kind(), RefKind::Branch);
        assert_eq!(r.short_name(), "main");
    }

    #[test]
    fn parse_tag() {
        let r: RefName = "tags/v1.0".parse().unwrap();
        assert_eq!(r.kind(), RefKind::Tag);
        assert_eq!(r.short_name(), "v1.0");
    }

    #[test]
    fn parse_missing_prefix() {
        assert!("main".parse::<RefName>().is_err());
    }

    #[test]
    fn parse_empty_short() {
        assert!("heads/".parse::<RefName>().is_err());
    }

    #[test]
    fn parse_invalid_chars() {
        assert!("heads/my branch".parse::<RefName>().is_err());
    }
}
