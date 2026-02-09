use crate::{BinaryError, BinaryOptions};

/// Binary container header (v1)
///
/// Fixed-size header structure:
/// - magic: [u8; 4] - file type identifier
/// - container_version: u16 - binary container version
/// - flags: u16 - reserved for future use
/// - schema_id: u64 - caller-defined schema identifier
/// - payload_len: u64 - length of payload in bytes
/// - checksum: u64 - fast non-cryptographic checksum
///
/// Total size: 4 + 2 + 2 + 8 + 8 + 8 = 32 bytes
#[derive(Debug, Clone, Copy)]
pub(crate) struct Header {
    pub magic: [u8; 4],
    pub container_version: u16,
    pub flags: u16,
    pub schema_id: u64,
    pub payload_len: u64,
    pub checksum: u64,
}

impl Header {
    pub const SIZE: usize = 32;

    /// Create a new header from options and payload
    pub fn new(opts: &BinaryOptions, payload: &[u8]) -> Self {
        let checksum = compute_checksum(payload);
        Self {
            magic: opts.magic,
            container_version: opts.container_version,
            flags: 0, // reserved
            schema_id: opts.schema_id,
            payload_len: payload.len() as u64,
            checksum,
        }
    }

    /// Serialize header to bytes (little-endian)
    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];

        // magic (4 bytes)
        bytes[0..4].copy_from_slice(&self.magic);

        // container_version (2 bytes)
        bytes[4..6].copy_from_slice(&self.container_version.to_le_bytes());

        // flags (2 bytes)
        bytes[6..8].copy_from_slice(&self.flags.to_le_bytes());

        // schema_id (8 bytes)
        bytes[8..16].copy_from_slice(&self.schema_id.to_le_bytes());

        // payload_len (8 bytes)
        bytes[16..24].copy_from_slice(&self.payload_len.to_le_bytes());

        // checksum (8 bytes)
        bytes[24..32].copy_from_slice(&self.checksum.to_le_bytes());

        bytes
    }

    /// Deserialize header from bytes (little-endian)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BinaryError> {
        if bytes.len() < Self::SIZE {
            return Err(BinaryError::Corrupt("Header too short"));
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);

        let container_version = u16::from_le_bytes([bytes[4], bytes[5]]);
        let flags = u16::from_le_bytes([bytes[6], bytes[7]]);

        let mut schema_id_bytes = [0u8; 8];
        schema_id_bytes.copy_from_slice(&bytes[8..16]);
        let schema_id = u64::from_le_bytes(schema_id_bytes);

        let mut payload_len_bytes = [0u8; 8];
        payload_len_bytes.copy_from_slice(&bytes[16..24]);
        let payload_len = u64::from_le_bytes(payload_len_bytes);

        let mut checksum_bytes = [0u8; 8];
        checksum_bytes.copy_from_slice(&bytes[24..32]);
        let checksum = u64::from_le_bytes(checksum_bytes);

        Ok(Self {
            magic,
            container_version,
            flags,
            schema_id,
            payload_len,
            checksum,
        })
    }

    /// Validate header against options
    pub fn validate(&self, opts: &BinaryOptions) -> Result<(), BinaryError> {
        // Check magic
        if self.magic != opts.magic {
            return Err(BinaryError::Incompatible("Magic mismatch"));
        }

        // Check container version
        if self.container_version != opts.container_version {
            return Err(BinaryError::Incompatible("Container version mismatch"));
        }

        // Check schema ID
        if self.schema_id != opts.schema_id {
            return Err(BinaryError::Incompatible("Schema ID mismatch"));
        }

        Ok(())
    }

    /// Validate checksum against payload
    pub fn validate_checksum(&self, payload: &[u8]) -> Result<(), BinaryError> {
        let computed = compute_checksum(payload);
        if computed != self.checksum {
            return Err(BinaryError::Corrupt("Checksum mismatch"));
        }
        Ok(())
    }
}

/// Compute a fast non-cryptographic checksum using FNV-1a
fn compute_checksum(data: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_round_trip() {
        let opts = BinaryOptions {
            magic: *b"TEST",
            container_version: 1,
            schema_id: 42,
            verify_checksum: true,
        };
        let payload = b"hello world";

        let header = Header::new(&opts, payload);
        let bytes = header.to_bytes();
        let decoded = Header::from_bytes(&bytes).unwrap();

        assert_eq!(header.magic, decoded.magic);
        assert_eq!(header.container_version, decoded.container_version);
        assert_eq!(header.schema_id, decoded.schema_id);
        assert_eq!(header.payload_len, decoded.payload_len);
        assert_eq!(header.checksum, decoded.checksum);
    }

    #[test]
    fn test_header_validation() {
        let opts = BinaryOptions {
            magic: *b"TEST",
            container_version: 1,
            schema_id: 42,
            verify_checksum: true,
        };
        let payload = b"hello world";

        let header = Header::new(&opts, payload);
        assert!(header.validate(&opts).is_ok());
        assert!(header.validate_checksum(payload).is_ok());
    }

    #[test]
    fn test_magic_mismatch() {
        let opts1 = BinaryOptions {
            magic: *b"TST1",
            container_version: 1,
            schema_id: 42,
            verify_checksum: true,
        };
        let opts2 = BinaryOptions {
            magic: *b"TST2",
            container_version: 1,
            schema_id: 42,
            verify_checksum: true,
        };
        let payload = b"hello world";

        let header = Header::new(&opts1, payload);
        assert!(matches!(
            header.validate(&opts2),
            Err(BinaryError::Incompatible("Magic mismatch"))
        ));
    }

    #[test]
    fn test_schema_id_mismatch() {
        let opts1 = BinaryOptions {
            magic: *b"TEST",
            container_version: 1,
            schema_id: 42,
            verify_checksum: true,
        };
        let opts2 = BinaryOptions {
            magic: *b"TEST",
            container_version: 1,
            schema_id: 99,
            verify_checksum: true,
        };
        let payload = b"hello world";

        let header = Header::new(&opts1, payload);
        assert!(matches!(
            header.validate(&opts2),
            Err(BinaryError::Incompatible("Schema ID mismatch"))
        ));
    }

    #[test]
    fn test_checksum_mismatch() {
        let opts = BinaryOptions {
            magic: *b"TEST",
            container_version: 1,
            schema_id: 42,
            verify_checksum: true,
        };
        let payload = b"hello world";
        let wrong_payload = b"hello earth";

        let header = Header::new(&opts, payload);
        assert!(matches!(
            header.validate_checksum(wrong_payload),
            Err(BinaryError::Corrupt("Checksum mismatch"))
        ));
    }

    #[test]
    fn test_checksum_computation() {
        let data1 = b"hello world";
        let data2 = b"hello world";
        let data3 = b"hello earth";

        let hash1 = compute_checksum(data1);
        let hash2 = compute_checksum(data2);
        let hash3 = compute_checksum(data3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
