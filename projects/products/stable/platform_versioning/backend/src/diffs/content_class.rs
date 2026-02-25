// projects/products/stable/platform_versioning/backend/src/diff/content_class.rs
use serde::{Deserialize, Serialize};

/// Classification of a file's content type.
///
/// # Binary detection policy
/// A file is classified as [`ContentClass::Binary`] if its first 8 KiB contain
/// a null byte (`\0`). Otherwise it is classified as [`ContentClass::Text`].
/// This heuristic matches the behavior of many well-known version control systems
/// and is deterministic for the same input bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentClass {
    /// Printable text content (no null bytes in the first 8 KiB).
    Text,
    /// Binary content (null byte found in the first 8 KiB).
    Binary,
}

impl ContentClass {
    /// The number of bytes to inspect for binary detection.
    pub const PROBE_LEN: usize = 8192;

    /// Classifies `bytes` as text or binary.
    pub fn of(bytes: &[u8]) -> Self {
        let probe = &bytes[..bytes.len().min(Self::PROBE_LEN)];
        if probe.contains(&0u8) {
            Self::Binary
        } else {
            Self::Text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_text() {
        assert_eq!(ContentClass::of(b"hello world\n"), ContentClass::Text);
    }

    #[test]
    fn null_byte_is_binary() {
        assert_eq!(ContentClass::of(b"hello\0world"), ContentClass::Binary);
    }

    #[test]
    fn empty_bytes_is_text() {
        assert_eq!(ContentClass::of(b""), ContentClass::Text);
    }
}
