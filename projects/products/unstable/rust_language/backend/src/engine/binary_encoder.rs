use crate::diagnostics::error::Error;
use crate::model::binary_format::BinaryFormat;

use common_binary::BinaryOptions;
use sha2::{Digest, Sha256};
use std::path::Path;

const BINARY_OPTIONS: BinaryOptions = BinaryOptions {
    magic: *b"RHLB",
    container_version: 1,
    schema_id: 1,
    verify_checksum: true,
};

pub struct BinaryEncoder;

impl BinaryEncoder {
    pub fn encode_rust_to_binary(rust_code: &str) -> Result<BinaryFormat, Error> {
        let payload = rust_code.as_bytes().to_vec();
        let checksum = Self::compute_checksum(&payload);
        Ok(BinaryFormat::new(payload, checksum))
    }

    pub fn write_binary(path: &Path, format: &BinaryFormat) -> Result<(), Error> {
        common_binary::write_binary(format, path, &BINARY_OPTIONS).map_err(Error::from)
    }

    pub fn read_binary(path: &Path) -> Result<BinaryFormat, Error> {
        let format: BinaryFormat =
            common_binary::read_binary(path, &BINARY_OPTIONS).map_err(Error::from)?;
        let expected = Self::compute_checksum(&format.payload);
        if format.checksum != expected {
            return Err(Error::Binary(format!(
                "payload checksum mismatch: expected {expected}, got {}",
                format.checksum
            )));
        }
        Ok(format)
    }

    fn compute_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}
