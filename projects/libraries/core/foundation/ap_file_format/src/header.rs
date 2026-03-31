use crate::content_type::ContentType;
use crate::error::ApFileError;

/// AP file format magic bytes: `APFF`
pub const AP_MAGIC: [u8; 4] = *b"APFF";

/// Current container version
pub const AP_CONTAINER_VERSION: u16 = 1;

/// AP file header (v1)
///
/// Fixed-size structure:
/// - magic: \[u8; 4\] – file type identifier (`APFF`)
/// - container_version: u16 – binary container version
/// - content_type: u16 – content payload type tag
/// - schema_id: u64 – caller-defined schema identifier
/// - payload_len: u64 – length of payload in bytes
/// - checksum: u64 – FNV-1a hash of payload
///
/// Total size: 4 + 2 + 2 + 8 + 8 + 8 = 32 bytes
#[derive(Debug, Clone, Copy)]
pub(crate) struct Header {
    pub magic: [u8; 4],
    pub container_version: u16,
    pub content_type: u16,
    pub schema_id: u64,
    pub payload_len: u64,
    pub checksum: u64,
}

impl Header {
    pub const SIZE: usize = 32;

    /// Create a new header for the given content type and payload.
    pub fn new(content_type: ContentType, schema_id: u64, payload: &[u8]) -> Self {
        Self {
            magic: AP_MAGIC,
            container_version: AP_CONTAINER_VERSION,
            content_type: content_type as u16,
            schema_id,
            payload_len: payload.len() as u64,
            checksum: compute_checksum(payload),
        }
    }

    /// Serialize header to bytes (little-endian).
    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..4].copy_from_slice(&self.magic);
        buf[4..6].copy_from_slice(&self.container_version.to_le_bytes());
        buf[6..8].copy_from_slice(&self.content_type.to_le_bytes());
        buf[8..16].copy_from_slice(&self.schema_id.to_le_bytes());
        buf[16..24].copy_from_slice(&self.payload_len.to_le_bytes());
        buf[24..32].copy_from_slice(&self.checksum.to_le_bytes());
        buf
    }

    /// Deserialize header from bytes (little-endian).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ApFileError> {
        if bytes.len() < Self::SIZE {
            return Err(ApFileError::Corrupt("Header too short"));
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);

        let container_version = u16::from_le_bytes([bytes[4], bytes[5]]);
        let content_type = u16::from_le_bytes([bytes[6], bytes[7]]);

        let mut buf8 = [0u8; 8];
        buf8.copy_from_slice(&bytes[8..16]);
        let schema_id = u64::from_le_bytes(buf8);

        buf8.copy_from_slice(&bytes[16..24]);
        let payload_len = u64::from_le_bytes(buf8);

        buf8.copy_from_slice(&bytes[24..32]);
        let checksum = u64::from_le_bytes(buf8);

        Ok(Self {
            magic,
            container_version,
            content_type,
            schema_id,
            payload_len,
            checksum,
        })
    }

    /// Validate magic and container version.
    pub fn validate(&self) -> Result<(), ApFileError> {
        if self.magic != AP_MAGIC {
            return Err(ApFileError::Incompatible("Magic mismatch"));
        }
        if self.container_version != AP_CONTAINER_VERSION {
            return Err(ApFileError::Incompatible("Container version mismatch"));
        }
        Ok(())
    }

    /// Validate the schema id matches expected.
    pub fn validate_schema(&self, expected: u64) -> Result<(), ApFileError> {
        if self.schema_id != expected {
            return Err(ApFileError::Incompatible("Schema ID mismatch"));
        }
        Ok(())
    }

    /// Validate checksum against payload.
    pub fn validate_checksum(&self, payload: &[u8]) -> Result<(), ApFileError> {
        let computed = compute_checksum(payload);
        if computed != self.checksum {
            return Err(ApFileError::Corrupt("Checksum mismatch"));
        }
        Ok(())
    }

    /// Get the content type enum for this header.
    pub fn content_type(&self) -> Option<ContentType> {
        ContentType::from_u16(self.content_type)
    }
}

/// Compute a fast non-cryptographic checksum using FNV-1a.
pub(crate) fn compute_checksum(data: &[u8]) -> u64 {
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
        let payload = b"hello world";
        let header = Header::new(ContentType::PlainText, 42, payload);
        let bytes = header.to_bytes();
        let decoded = Header::from_bytes(&bytes).unwrap();

        assert_eq!(header.magic, decoded.magic);
        assert_eq!(header.container_version, decoded.container_version);
        assert_eq!(header.content_type, decoded.content_type);
        assert_eq!(header.schema_id, decoded.schema_id);
        assert_eq!(header.payload_len, decoded.payload_len);
        assert_eq!(header.checksum, decoded.checksum);
    }

    #[test]
    fn test_header_validation() {
        let payload = b"hello";
        let header = Header::new(ContentType::Binary, 1, payload);
        assert!(header.validate().is_ok());
        assert!(header.validate_checksum(payload).is_ok());
        assert!(header.validate_schema(1).is_ok());
    }

    #[test]
    fn test_magic_mismatch() {
        let payload = b"data";
        let mut header = Header::new(ContentType::Binary, 0, payload);
        header.magic = *b"XXXX";
        assert!(matches!(
            header.validate(),
            Err(ApFileError::Incompatible("Magic mismatch"))
        ));
    }

    #[test]
    fn test_checksum_mismatch() {
        let payload = b"hello world";
        let header = Header::new(ContentType::Binary, 0, payload);
        assert!(matches!(
            header.validate_checksum(b"hello earth"),
            Err(ApFileError::Corrupt("Checksum mismatch"))
        ));
    }

    #[test]
    fn test_schema_mismatch() {
        let payload = b"data";
        let header = Header::new(ContentType::Binary, 10, payload);
        assert!(matches!(
            header.validate_schema(20),
            Err(ApFileError::Incompatible("Schema ID mismatch"))
        ));
    }

    #[test]
    fn test_content_type_round_trip() {
        let header = Header::new(ContentType::Json, 0, b"{}");
        assert_eq!(header.content_type(), Some(ContentType::Json));
    }

    #[test]
    fn test_checksum_deterministic() {
        let d1 = b"hello world";
        let d2 = b"hello world";
        let d3 = b"hello earth";
        assert_eq!(compute_checksum(d1), compute_checksum(d2));
        assert_ne!(compute_checksum(d1), compute_checksum(d3));
    }
}
