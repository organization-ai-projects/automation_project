use std::path::Path;

use crate::bundle::bundle_unpacker::read_header_and_manifest;
use crate::diagnostics::Error;
use crate::hash::FileHasher;
use crate::verify::verify_report::{EntryResult, EntryStatus, VerifyReport};

pub struct Verifier;

impl Verifier {
    pub fn verify(bundle: &Path) -> Result<VerifyReport, Error> {
        let data = std::fs::read(bundle)?;
        let (manifest, mut cursor) = read_header_and_manifest(&data, 0)?;

        let mut results = Vec::new();
        let mut all_ok = true;

        for entry in &manifest.entries {
            // Read content length
            if data.len() < cursor + 8 {
                return Err(Error::ManifestFormat(format!(
                    "bundle truncated before content of '{}'",
                    entry.path
                )));
            }
            let content_len =
                u64::from_le_bytes(data[cursor..cursor + 8].try_into().unwrap()) as usize;
            cursor += 8;

            if data.len() < cursor + content_len {
                return Err(Error::ManifestFormat(format!(
                    "bundle truncated inside content of '{}'",
                    entry.path
                )));
            }
            let content = &data[cursor..cursor + content_len];
            cursor += content_len;

            let actual_size = content_len as u64;
            if actual_size != entry.size {
                all_ok = false;
                results.push(EntryResult {
                    path: entry.path.clone(),
                    status: EntryStatus::SizeMismatch {
                        expected: entry.size,
                        actual: actual_size,
                    },
                });
                continue;
            }

            let actual_hash = FileHasher::hash_bytes(content);
            if actual_hash != entry.hash {
                all_ok = false;
                results.push(EntryResult {
                    path: entry.path.clone(),
                    status: EntryStatus::HashMismatch {
                        expected: entry.hash.clone(),
                        actual: actual_hash,
                    },
                });
                continue;
            }

            results.push(EntryResult {
                path: entry.path.clone(),
                status: EntryStatus::Ok,
            });
        }

        Ok(VerifyReport {
            ok: all_ok,
            entry_count: manifest.entries.len(),
            results,
        })
    }
}
