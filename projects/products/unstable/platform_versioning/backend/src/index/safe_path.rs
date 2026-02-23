// projects/products/unstable/platform_versioning/backend/src/index/safe_path.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;

/// A validated, safe relative path for use in an index or tree.
///
/// # Safety rules (enforced at construction)
/// - Must not be empty.
/// - Must not contain `..` components.
/// - Must not start with `/`.
/// - Must not contain null bytes.
/// - Must not contain backslashes (platform-agnostic safety).
/// - Must not contain `//` (double slash).
/// - Components must not be `.` (current-directory references are meaningless).
/// - Maximum length: 4096 bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SafePath(String);

impl SafePath {
    /// Maximum byte length.
    pub const MAX_LEN: usize = 4096;

    /// Returns the path string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(s: &str) -> Result<(), PvError> {
        if s.is_empty() {
            return Err(PvError::UnsafePath("path must not be empty".to_string()));
        }
        if s.len() > Self::MAX_LEN {
            return Err(PvError::UnsafePath(format!(
                "path exceeds {} bytes",
                Self::MAX_LEN
            )));
        }
        if s.starts_with('/') {
            return Err(PvError::UnsafePath(
                "path must not start with '/'".to_string(),
            ));
        }
        if s.contains('\0') {
            return Err(PvError::UnsafePath("path contains null byte".to_string()));
        }
        if s.contains('\\') {
            return Err(PvError::UnsafePath(
                "path must not contain backslashes".to_string(),
            ));
        }
        if s.contains("//") {
            return Err(PvError::UnsafePath(
                "path must not contain '//'".to_string(),
            ));
        }
        for component in s.split('/') {
            if component == ".." {
                return Err(PvError::UnsafePath(
                    "path must not contain '..' components".to_string(),
                ));
            }
            if component == "." {
                return Err(PvError::UnsafePath(
                    "path must not contain '.' components".to_string(),
                ));
            }
            if component.is_empty() {
                return Err(PvError::UnsafePath(
                    "path component must not be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl fmt::Display for SafePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for SafePath {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_simple() {
        let p: SafePath = "src/main.rs".parse().unwrap();
        assert_eq!(p.as_str(), "src/main.rs");
    }

    #[test]
    fn reject_traversal() {
        assert!("../etc/passwd".parse::<SafePath>().is_err());
        assert!("foo/../../etc/passwd".parse::<SafePath>().is_err());
    }

    #[test]
    fn reject_absolute() {
        assert!("/etc/passwd".parse::<SafePath>().is_err());
    }

    #[test]
    fn reject_null_byte() {
        assert!("foo\0bar".parse::<SafePath>().is_err());
    }

    #[test]
    fn reject_backslash() {
        assert!("foo\\bar".parse::<SafePath>().is_err());
    }

    #[test]
    fn reject_double_slash() {
        assert!("foo//bar".parse::<SafePath>().is_err());
    }

    #[test]
    fn reject_dot_component() {
        assert!("foo/./bar".parse::<SafePath>().is_err());
    }

    #[test]
    fn reject_empty() {
        assert!("".parse::<SafePath>().is_err());
    }
}
