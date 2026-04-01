use std::path::Path;

use crate::bundle::bundle_format::{FORMAT_VERSION, MAGIC};
use crate::diagnostics::Error;
use crate::manifest::Manifest;

pub struct BundleUnpacker;

impl BundleUnpacker {
    pub fn unpack(bundle: &Path, out: &Path) -> Result<(), Error> {
        let data = std::fs::read(bundle)?;
        let mut cursor = 0usize;

        let (manifest, offset) = read_header_and_manifest(&data, cursor)?;
        cursor = offset;

        std::fs::create_dir_all(out)?;

        for entry in &manifest.entries {
            let content = read_file_content(&data, &mut cursor)?;

            if content.len() as u64 != entry.size {
                return Err(Error::ManifestFormat(format!(
                    "size mismatch for '{}': manifest={}, actual={}",
                    entry.path,
                    entry.size,
                    content.len()
                )));
            }

            let dest = out.join(entry.path.replace('/', std::path::MAIN_SEPARATOR_STR));
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&dest, &content)?;
        }

        Ok(())
    }
}

pub(crate) fn read_header_and_manifest(
    data: &[u8],
    mut cursor: usize,
) -> Result<(Manifest, usize), Error> {
    // Magic
    if data.len() < cursor + 4 {
        return Err(Error::ManifestFormat("bundle too short: missing magic".to_string()));
    }
    let magic = &data[cursor..cursor + 4];
    if magic != MAGIC {
        return Err(Error::ManifestFormat(format!(
            "invalid magic bytes: expected {:?}, got {:?}",
            MAGIC, magic
        )));
    }
    cursor += 4;

    // Version
    if data.len() < cursor + 4 {
        return Err(Error::ManifestFormat("bundle too short: missing version".to_string()));
    }
    let version = u32::from_be_bytes(data[cursor..cursor + 4].try_into().unwrap());
    if version != FORMAT_VERSION {
        return Err(Error::ManifestFormat(format!(
            "unsupported bundle version {version} (expected {FORMAT_VERSION})"
        )));
    }
    cursor += 4;

    // Manifest length
    if data.len() < cursor + 8 {
        return Err(Error::ManifestFormat(
            "bundle too short: missing manifest length".to_string(),
        ));
    }
    let manifest_len =
        u64::from_le_bytes(data[cursor..cursor + 8].try_into().unwrap()) as usize;
    cursor += 8;

    // Manifest JSON
    if data.len() < cursor + manifest_len {
        return Err(Error::ManifestFormat("bundle too short: truncated manifest".to_string()));
    }
    let manifest_json = std::str::from_utf8(&data[cursor..cursor + manifest_len])
        .map_err(|e| Error::ManifestFormat(format!("manifest is not valid UTF-8: {e}")))?;
    cursor += manifest_len;

    let manifest = Manifest::from_canonical_json(manifest_json)?;

    Ok((manifest, cursor))
}

fn read_file_content(data: &[u8], cursor: &mut usize) -> Result<Vec<u8>, Error> {
    if data.len() < *cursor + 8 {
        return Err(Error::ManifestFormat(
            "bundle too short: missing content length".to_string(),
        ));
    }
    let content_len =
        u64::from_le_bytes(data[*cursor..*cursor + 8].try_into().unwrap()) as usize;
    *cursor += 8;

    if data.len() < *cursor + content_len {
        return Err(Error::ManifestFormat("bundle too short: truncated content".to_string()));
    }
    let content = data[*cursor..*cursor + content_len].to_vec();
    *cursor += content_len;

    Ok(content)
}
