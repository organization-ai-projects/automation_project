use crate::diagnostics::error::Error;
use crate::model::binary_format::BinaryFormat;

use common_binary::BinaryOptions;
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct BinaryEncoder;

impl BinaryEncoder {
    pub fn encode_ast_to_binary(rust_code: &str) -> Result<BinaryFormat, Error> {
        let payload = rust_code.as_bytes().to_vec();
        let checksum = Self::compute_checksum(&payload);
        Ok(BinaryFormat::new(payload, checksum))
    }

    pub fn write_binary(path: &Path, format: &BinaryFormat) -> Result<(), Error> {
        let opts = BinaryOptions {
            magic: *b"RHLB",
            container_version: 1,
            schema_id: 1,
            verify_checksum: true,
        };
        common_binary::write_binary(format, path, &opts).map_err(Error::from)
    }

    pub fn read_binary(path: &Path) -> Result<BinaryFormat, Error> {
        let opts = BinaryOptions {
            magic: *b"RHLB",
            container_version: 1,
            schema_id: 1,
            verify_checksum: true,
        };
        common_binary::read_binary(path, &opts).map_err(Error::from)
    }

    fn compute_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}
