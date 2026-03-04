use crate::diagnostics::backend_error::BackendError;
use std::io::Write;

pub struct CanonicalWriter;

impl CanonicalWriter {
    pub fn write_all(writer: &mut dyn Write, bytes: &[u8]) -> Result<(), BackendError> {
        writer
            .write_all(bytes)
            .map_err(|error| BackendError::Io(error.to_string()))
    }
}
