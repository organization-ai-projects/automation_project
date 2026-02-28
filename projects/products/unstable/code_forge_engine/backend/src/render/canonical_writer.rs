// projects/products/unstable/code_forge_engine/backend/src/render/canonical_writer.rs
use crate::diagnostics::error::ForgeError;
use std::io::Write;

pub struct CanonicalWriter;

impl CanonicalWriter {
    pub fn write_all(writer: &mut dyn Write, bytes: &[u8]) -> Result<(), ForgeError> {
        writer
            .write_all(bytes)
            .map_err(|e| ForgeError::Io(e.to_string()))
    }
}
