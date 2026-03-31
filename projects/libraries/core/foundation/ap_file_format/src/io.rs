use crate::content_type::ContentType;
use crate::error::{ApFileError, ApFileResult};
use crate::header::Header;
use crate::image::ImageData;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// Options controlling how an AP file is written or read.
#[derive(Debug, Clone)]
pub struct ApFileOptions {
    /// Caller-defined schema identifier.
    pub schema_id: u64,
    /// Whether to verify the checksum on read.
    pub verify_checksum: bool,
}

impl Default for ApFileOptions {
    fn default() -> Self {
        Self {
            schema_id: 0,
            verify_checksum: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Write helpers
// ---------------------------------------------------------------------------

/// Write plain UTF-8 text to an AP file.
pub fn write_text(
    path: impl AsRef<Path>,
    text: &str,
    opts: &ApFileOptions,
) -> ApFileResult<()> {
    write_payload(path, ContentType::PlainText, opts, text.as_bytes())
}

/// Write Markdown text to an AP file.
pub fn write_markdown(
    path: impl AsRef<Path>,
    text: &str,
    opts: &ApFileOptions,
) -> ApFileResult<()> {
    write_payload(path, ContentType::Markdown, opts, text.as_bytes())
}

/// Write a serde-serializable value as JSON to an AP file.
pub fn write_json<T: Serialize>(
    path: impl AsRef<Path>,
    value: &T,
    opts: &ApFileOptions,
) -> ApFileResult<()> {
    let json_str = common_json::to_json_string_pretty(value)
        .map_err(|e| ApFileError::Encode(e.to_string()))?;
    write_payload(path, ContentType::Json, opts, json_str.as_bytes())
}

/// Write a serde-serializable value as RON to an AP file.
pub fn write_ron<T: Serialize>(
    path: impl AsRef<Path>,
    value: &T,
    opts: &ApFileOptions,
) -> ApFileResult<()> {
    let ron_str = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())
        .map_err(|e| ApFileError::Encode(e.to_string()))?;
    write_payload(path, ContentType::Ron, opts, ron_str.as_bytes())
}

/// Write raw binary data to an AP file.
pub fn write_binary(
    path: impl AsRef<Path>,
    data: &[u8],
    opts: &ApFileOptions,
) -> ApFileResult<()> {
    write_payload(path, ContentType::Binary, opts, data)
}

/// Write an image to an AP file.
pub fn write_image(
    path: impl AsRef<Path>,
    image: &ImageData,
    opts: &ApFileOptions,
) -> ApFileResult<()> {
    let payload = image.to_payload();
    write_payload(path, ContentType::Image, opts, &payload)
}

// ---------------------------------------------------------------------------
// Read helpers
// ---------------------------------------------------------------------------

/// Read plain text from an AP file.
pub fn read_text(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<String> {
    let (content_type, payload) = read_raw(path, opts)?;
    if content_type != ContentType::PlainText {
        return Err(ApFileError::InvalidContentType(format!(
            "Expected PlainText, found {:?}",
            content_type
        )));
    }
    String::from_utf8(payload).map_err(|e| ApFileError::Decode(e.to_string()))
}

/// Read Markdown from an AP file.
pub fn read_markdown(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<String> {
    let (content_type, payload) = read_raw(path, opts)?;
    if content_type != ContentType::Markdown {
        return Err(ApFileError::InvalidContentType(format!(
            "Expected Markdown, found {:?}",
            content_type
        )));
    }
    String::from_utf8(payload).map_err(|e| ApFileError::Decode(e.to_string()))
}

/// Read a JSON-encoded value from an AP file.
pub fn read_json<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<T> {
    let (content_type, payload) = read_raw(path, opts)?;
    if content_type != ContentType::Json {
        return Err(ApFileError::InvalidContentType(format!(
            "Expected Json, found {:?}",
            content_type
        )));
    }
    let text = std::str::from_utf8(&payload)
        .map_err(|e| ApFileError::Decode(e.to_string()))?;
    common_json::from_json_str(text).map_err(|e| ApFileError::Decode(e.to_string()))
}

/// Read a RON-encoded value from an AP file.
pub fn read_ron<T: DeserializeOwned>(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<T> {
    let (content_type, payload) = read_raw(path, opts)?;
    if content_type != ContentType::Ron {
        return Err(ApFileError::InvalidContentType(format!(
            "Expected Ron, found {:?}",
            content_type
        )));
    }
    let text = std::str::from_utf8(&payload)
        .map_err(|e| ApFileError::Decode(e.to_string()))?;
    common_ron::read_ron_str(text).map_err(|e| ApFileError::Decode(e.to_string()))
}

/// Read raw binary data from an AP file.
pub fn read_binary(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<Vec<u8>> {
    let (content_type, payload) = read_raw(path, opts)?;
    if content_type != ContentType::Binary {
        return Err(ApFileError::InvalidContentType(format!(
            "Expected Binary, found {:?}",
            content_type
        )));
    }
    Ok(payload)
}

/// Read an image from an AP file.
pub fn read_image(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<ImageData> {
    let (content_type, payload) = read_raw(path, opts)?;
    if content_type != ContentType::Image {
        return Err(ApFileError::InvalidContentType(format!(
            "Expected Image, found {:?}",
            content_type
        )));
    }
    ImageData::from_payload(&payload)
}

/// Read an AP file returning the content type and raw payload bytes.
pub fn read_raw(
    path: impl AsRef<Path>,
    opts: &ApFileOptions,
) -> ApFileResult<(ContentType, Vec<u8>)> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    if contents.len() < Header::SIZE {
        return Err(ApFileError::Corrupt("File too short for header"));
    }

    let header = Header::from_bytes(&contents[..Header::SIZE])?;
    header.validate()?;
    header.validate_schema(opts.schema_id)?;

    let payload_start = Header::SIZE;
    let payload_len = usize::try_from(header.payload_len)
        .map_err(|_| ApFileError::Corrupt("Payload length does not fit platform usize"))?;
    let payload_end = payload_start
        .checked_add(payload_len)
        .ok_or(ApFileError::Corrupt("Payload length overflow"))?;

    if contents.len() < payload_end {
        return Err(ApFileError::Corrupt("File too short for payload"));
    }

    let payload = &contents[payload_start..payload_end];

    if opts.verify_checksum {
        header.validate_checksum(payload)?;
    }

    let content_type = header
        .content_type()
        .ok_or(ApFileError::InvalidContentType(format!(
            "Unknown content type tag: {}",
            header.content_type
        )))?;

    Ok((content_type, payload.to_vec()))
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn write_payload(
    path: impl AsRef<Path>,
    content_type: ContentType,
    opts: &ApFileOptions,
    payload: &[u8],
) -> ApFileResult<()> {
    let header = Header::new(content_type, opts.schema_id, payload);
    let header_bytes = header.to_bytes();

    let target_path = path.as_ref();
    let (mut file, temp_path) = create_temp_file_near(target_path)?;

    if let Err(err) = (|| -> ApFileResult<()> {
        file.write_all(&header_bytes)?;
        file.write_all(payload)?;
        file.sync_all()?;
        Ok(())
    })() {
        drop(file);
        let _ = fs::remove_file(&temp_path);
        return Err(err);
    }

    drop(file);
    if let Err(err) = replace_file(&temp_path, target_path) {
        let _ = fs::remove_file(&temp_path);
        return Err(ApFileError::Io(err));
    }

    sync_parent_dir(target_path);

    Ok(())
}

fn create_temp_file_near(target_path: &Path) -> Result<(File, PathBuf), ApFileError> {
    let parent = target_path.parent().unwrap_or(Path::new("."));
    let file_name = target_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("apfile");

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
            Err(err) => return Err(ApFileError::Io(err)),
        }
    }

    Err(ApFileError::Io(std::io::Error::new(
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
                let mut backup = dst.to_path_buf();
                let mut found_backup = false;
                for i in 0..16 {
                    let candidate = if i == 0 {
                        backup.with_extension("bak")
                    } else {
                        backup.with_extension(format!("bak{i}"))
                    };
                    if !candidate.exists() {
                        backup = candidate;
                        found_backup = true;
                        break;
                    }
                }
                if !found_backup {
                    return Err(err);
                }
                fs::rename(dst, &backup)?;
                match fs::rename(src, dst) {
                    Ok(()) => {
                        let _ = fs::remove_file(&backup);
                        Ok(())
                    }
                    Err(replace_err) => {
                        let _ = fs::rename(&backup, dst);
                        Err(replace_err)
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    #[cfg(not(windows))]
    {
        fs::rename(src, dst)
    }
}

#[cfg(unix)]
fn sync_parent_dir(target_path: &Path) {
    if let Some(parent) = target_path.parent()
        && let Ok(dir) = File::open(parent)
    {
        let _ = dir.sync_all();
    }
}

#[cfg(not(unix))]
fn sync_parent_dir(_target_path: &Path) {}
