use crate::header::Header;
use crate::{BinaryDecode, BinaryEncode, BinaryError, BinaryOptions};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

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

    // Write to a temp file first, then atomically rename.
    let target_path = path.as_ref();
    let (mut file, temp_path) = create_temp_file_near(target_path)?;

    if let Err(err) = (|| -> Result<(), BinaryError> {
        file.write_all(&header_bytes)?;
        file.write_all(&payload)?;
        file.sync_all()?;
        Ok(())
    })() {
        // Ensure the file handle is closed before attempting to remove the temp file.
        drop(file);
        let _ = fs::remove_file(&temp_path);
        return Err(err);
    }

    drop(file);
    if let Err(err) = replace_file(&temp_path, target_path) {
        let _ = fs::remove_file(&temp_path);
        return Err(BinaryError::Io(err));
    }

    // Best-effort directory sync improves rename durability on filesystems that require it.
    sync_parent_dir(target_path);

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
    let payload_len = usize::try_from(header.payload_len)
        .map_err(|_| BinaryError::Corrupt("Payload length does not fit platform usize"))?;
    let payload_end = payload_start
        .checked_add(payload_len)
        .ok_or(BinaryError::Corrupt("Payload length overflow"))?;

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

fn create_temp_file_near(target_path: &Path) -> Result<(File, PathBuf), BinaryError> {
    let parent = target_path.parent().unwrap_or(Path::new("."));
    let file_name = target_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("binary");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    for attempt in 0..32 {
        let temp_name = format!(".{file_name}.tmp-{}-{timestamp}-{attempt}", process::id());
        let temp_path = parent.join(temp_name);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        {
            Ok(file) => return Ok((file, temp_path)),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(BinaryError::Io(err)),
        }
    }

    Err(BinaryError::Io(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "failed to create unique temp file",
    )))
}

fn replace_file(src: &Path, dst: &Path) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        match fs::rename(src, dst) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                fs::remove_file(dst)?;
                fs::rename(src, dst)
            }
            Err(err) => Err(err),
        }
    }

    #[cfg(not(windows))]
    {
        fs::rename(src, dst)
    }
}

fn sync_parent_dir(target_path: &Path) {
    #[cfg(unix)]
    {
        if let Some(parent) = target_path.parent()
            && let Ok(dir) = File::open(parent)
        {
            let _ = dir.sync_all();
        }
    }
}
