use crate::diagnostics::engine_error::EngineError;
use common_binary::{BinaryOptions, read_binary, write_binary};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

const MAGIC: [u8; 4] = *b"USIM";
const SCHEMA_ID: u64 = 1;

fn options() -> BinaryOptions {
    BinaryOptions {
        magic: MAGIC,
        container_version: 1,
        schema_id: SCHEMA_ID,
        verify_checksum: true,
    }
}

pub fn save_binary<T: Serialize>(value: &T, path: &Path) -> Result<(), EngineError> {
    write_binary(value, path, &options()).map_err(|e| EngineError::BinaryCodec(e.to_string()))
}

pub fn load_binary<T: DeserializeOwned>(path: &Path) -> Result<T, EngineError> {
    read_binary(path, &options()).map_err(|e| EngineError::BinaryCodec(e.to_string()))
}
