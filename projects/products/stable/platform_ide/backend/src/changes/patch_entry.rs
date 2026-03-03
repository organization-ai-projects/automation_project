use serde::{Deserialize, Serialize};

use crate::slices::AllowedPath;

/// A single patch entry within a change set.
///
/// Each entry records the new content for one allowed file. Only files whose
/// paths have been validated through the slice manifest can appear here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEntry {
    /// The validated path of the file.
    pub path: AllowedPath,
    /// The full new content for this file (hex-encoded when serialized).
    #[serde(with = "hex_bytes")]
    pub content: Vec<u8>,
}

/// Serde module for hex encoding of byte vectors.
mod hex_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        encoded.serialize(s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        (0..s.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&s[i..i + 2], 16)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))
            })
            .collect()
    }
}
