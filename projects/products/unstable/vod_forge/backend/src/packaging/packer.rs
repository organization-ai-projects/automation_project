use sha2::{Digest, Sha256};
use crate::diagnostics::BackendError;
use crate::packaging::asset_id::AssetId;
use crate::packaging::asset_manifest::{AssetManifest, ChunkEntry};

pub struct Packer;

impl Packer {
    pub fn pack(
        mut input_files: Vec<(String, Vec<u8>)>,
        bundle_id: &str,
    ) -> Result<(AssetManifest, Vec<u8>), BackendError> {
        // Sort by filename for determinism
        input_files.sort_by(|a, b| a.0.cmp(&b.0));

        let mut chunks: Vec<ChunkEntry> = Vec::new();
        let mut data_bytes: Vec<u8> = Vec::new();
        let mut offset: u64 = 0;

        for (filename, bytes) in &input_files {
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let sha256 = hex::encode(hasher.finalize());
            let length = bytes.len() as u64;
            chunks.push(ChunkEntry {
                asset_id: AssetId::from(filename.as_str()),
                offset,
                length,
                sha256,
            });
            data_bytes.extend_from_slice(bytes);
            offset += length;
        }

        // Chunks are already in sorted order (input_files was sorted by filename)
        let manifest = AssetManifest {
            bundle_id: bundle_id.to_string(),
            chunks,
        };

        // Bundle = magic(4) + manifest_len(4 LE) + manifest_json + chunk_data
        // Use canonical JSON serialization to guarantee deterministic field order
        let manifest_json = manifest.to_canonical_json();
        let manifest_bytes = manifest_json.as_bytes();
        let manifest_len = manifest_bytes.len() as u32;

        let mut bundle: Vec<u8> = Vec::new();
        bundle.extend_from_slice(b"VBUN");
        bundle.extend_from_slice(&manifest_len.to_le_bytes());
        bundle.extend_from_slice(manifest_bytes);
        bundle.extend_from_slice(&data_bytes);

        Ok((manifest, bundle))
    }
}
