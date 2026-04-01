use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryFormat {
    pub magic: [u8; 4],
    pub version: u8,
    pub checksum: String,
    pub payload: Vec<u8>,
}

impl BinaryFormat {
    pub fn new(payload: Vec<u8>, checksum: String) -> Self {
        Self {
            magic: *b"RHLB",
            version: 1,
            checksum,
            payload,
        }
    }

    pub fn validate_magic(&self) -> bool {
        self.magic == *b"RHLB"
    }
}
