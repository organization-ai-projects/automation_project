/// Options for binary persistence operations.
#[derive(Debug, Clone)]
pub struct BinaryOptions {
    /// File type identifier (4 bytes)
    pub magic: [u8; 4],

    /// Binary container version
    pub container_version: u16,

    /// Caller-defined schema identifier
    pub schema_id: u64,

    /// Whether to verify checksum on read
    pub verify_checksum: bool,
}

impl Default for BinaryOptions {
    fn default() -> Self {
        Self {
            magic: *b"CBIN",
            container_version: 1,
            schema_id: 0,
            verify_checksum: true,
        }
    }
}
