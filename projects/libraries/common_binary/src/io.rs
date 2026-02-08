use crate::header::Header;
use crate::{BinaryDecode, BinaryEncode, BinaryError, BinaryOptions};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Write a value to a file in binary format.
///
/// The file will contain:
/// 1. A fixed-size header with metadata and checksum
/// 2. The encoded payload
///
/// # Errors
///
/// Returns `BinaryError` if:
/// - The value cannot be encoded
/// - The file cannot be created or written
///
/// # Example
///
/// ```rust,no_run
/// use common_binary::{BinaryOptions, write_binary};
/// # use serde::Serialize;
/// # #[derive(Serialize)]
/// # struct MyData;
/// # fn example() -> Result<(), common_binary::BinaryError> {
/// let data = MyData;
/// let opts = BinaryOptions {
///     magic: *b"MYDT",
///     container_version: 1,
///     schema_id: 1,
///     verify_checksum: true,
/// };
/// write_binary(&data, "data.bin", &opts)?;
/// # Ok(())
/// # }
/// ```
pub fn write_binary<T: BinaryEncode>(
    value: &T,
    path: impl AsRef<Path>,
    opts: &BinaryOptions,
) -> Result<(), BinaryError> {
    // Encode payload
    let mut payload = Vec::new();
    value.encode_binary(&mut payload)?;

    // Create header
    let header = Header::new(opts, &payload);
    let header_bytes = header.to_bytes();

    // Write to file
    let mut file = File::create(path)?;
    file.write_all(&header_bytes)?;
    file.write_all(&payload)?;
    file.sync_all()?;

    Ok(())
}

/// Read a value from a file in binary format.
///
/// The file must contain:
/// 1. A valid header matching the provided options
/// 2. A valid payload that can be decoded
///
/// # Errors
///
/// Returns `BinaryError` if:
/// - The file cannot be read
/// - The header is invalid or doesn't match options
/// - The checksum doesn't match (if `verify_checksum` is true)
/// - The payload cannot be decoded
///
/// # Example
///
/// ```rust,no_run
/// use common_binary::{BinaryOptions, read_binary};
/// # use serde::Deserialize;
/// # #[derive(Deserialize)]
/// # struct MyData;
/// # fn example() -> Result<(), common_binary::BinaryError> {
/// let opts = BinaryOptions {
///     magic: *b"MYDT",
///     container_version: 1,
///     schema_id: 1,
///     verify_checksum: true,
/// };
/// let data: MyData = read_binary("data.bin", &opts)?;
/// # Ok(())
/// # }
/// ```
pub fn read_binary<T: BinaryDecode>(
    path: impl AsRef<Path>,
    opts: &BinaryOptions,
) -> Result<T, BinaryError> {
    // Read entire file
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Parse header
    if contents.len() < Header::SIZE {
        return Err(BinaryError::Corrupt("File too short for header"));
    }

    let header = Header::from_bytes(&contents[..Header::SIZE])?;

    // Validate header
    header.validate(opts)?;

    // Extract payload
    let payload_start = Header::SIZE;
    let payload_end = payload_start + header.payload_len as usize;

    if contents.len() < payload_end {
        return Err(BinaryError::Corrupt("File too short for payload"));
    }

    let payload = &contents[payload_start..payload_end];

    // Validate checksum if requested
    if opts.verify_checksum {
        header.validate_checksum(payload)?;
    }

    // Decode payload
    T::decode_binary(payload)
}
