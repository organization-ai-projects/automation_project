use std::io::Write;
use std::path::Path;

use crate::bundle::bundle_format::{FORMAT_VERSION, MAGIC};
use crate::diagnostics::Error;
use crate::hash::FileHasher;
use crate::manifest::{Manifest, ManifestEntry};

pub struct BundlePacker;

impl BundlePacker {
    pub fn pack(root: &Path, out: &Path) -> Result<(), Error> {
        let entries = Self::collect_entries(root)?;
        let manifest = Manifest::new(entries);
        let manifest_json = manifest.to_canonical_json();
        let manifest_bytes = manifest_json.as_bytes();

        let mut data: Vec<u8> = Vec::new();

        // Header
        data.extend_from_slice(&MAGIC);
        data.extend_from_slice(&FORMAT_VERSION.to_be_bytes());

        // Manifest length + manifest JSON
        let manifest_len = manifest_bytes.len() as u64;
        data.extend_from_slice(&manifest_len.to_le_bytes());
        data.extend_from_slice(manifest_bytes);

        // File contents in canonical (sorted) order
        for entry in &manifest.entries {
            let file_path = root.join(entry.path.replace('/', std::path::MAIN_SEPARATOR_STR));
            let content = std::fs::read(&file_path)
                .map_err(|e| Error::Internal(format!("failed to read '{}': {e}", entry.path)))?;
            let content_len = content.len() as u64;
            data.extend_from_slice(&content_len.to_le_bytes());
            data.extend_from_slice(&content);
        }

        if let Some(parent) = out.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let mut file = std::fs::File::create(out)?;
        file.write_all(&data)?;

        Ok(())
    }

    fn collect_entries(root: &Path) -> Result<Vec<ManifestEntry>, Error> {
        let mut entries = Vec::new();
        Self::walk(root, root, &mut entries)?;
        Ok(entries)
    }

    fn walk(root: &Path, current: &Path, entries: &mut Vec<ManifestEntry>) -> Result<(), Error> {
        let mut dir_entries: Vec<std::path::PathBuf> = std::fs::read_dir(current)
            .map_err(|e| {
                Error::Internal(format!("failed to read dir '{}': {e}", current.display()))
            })?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();

        // Sort for deterministic OS-independent ordering
        dir_entries.sort();

        for path in dir_entries {
            if path.is_dir() {
                Self::walk(root, &path, entries)?;
            } else if path.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|e| Error::Internal(e.to_string()))?;
                let rel_str = relative
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/");

                let content = std::fs::read(&path)?;
                let hash = FileHasher::hash_bytes(&content);
                let size = content.len() as u64;

                entries.push(ManifestEntry {
                    hash,
                    path: rel_str,
                    size,
                });
            }
        }

        Ok(())
    }
}
