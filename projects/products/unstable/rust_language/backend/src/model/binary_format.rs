use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BinaryFormat {
    pub(crate) magic: [u8; 4],
    pub(crate) version: u8,
    pub(crate) checksum: String,
    pub(crate) payload: Vec<u8>,
}

impl BinaryFormat {
    pub(crate) fn new(payload: Vec<u8>, checksum: String) -> Self {
        Self {
            magic: *b"RHLB",
            version: 1,
            checksum,
            payload,
        }
    }

    pub(crate) fn validate_magic(&self) -> bool {
        self.magic == *b"RHLB"
    }
}
