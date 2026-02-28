use sha2::{Digest, Sha256};
use crate::diagnostics::BackendError;
use crate::packaging::asset_id::AssetId;
use crate::packaging::asset_manifest::AssetManifest;
use crate::io::JsonCodec;

pub struct Unpacker;

impl Unpacker {
    pub fn unpack(bundle: &[u8]) -> Result<(AssetManifest, Vec<(AssetId, Vec<u8>)>), BackendError> {
        if bundle.len() < 8 {
            return Err(BackendError::Packaging("bundle too short".to_string()));
        }
        if &bundle[..4] != b"VBUN" {
            return Err(BackendError::Packaging("invalid magic".to_string()));
        }
        let manifest_len =
            u32::from_le_bytes([bundle[4], bundle[5], bundle[6], bundle[7]]) as usize;
        let manifest_start = 8;
        let manifest_end = manifest_start + manifest_len;
        if bundle.len() < manifest_end {
            return Err(BackendError::Packaging("bundle truncated at manifest".to_string()));
        }
        let manifest_str = std::str::from_utf8(&bundle[manifest_start..manifest_end])
            .map_err(|e| BackendError::Packaging(e.to_string()))?;
        let manifest: AssetManifest = JsonCodec::decode(manifest_str)
            .map_err(|e| BackendError::Packaging(e.to_string()))?;

        let data_start = manifest_end;
        let mut assets: Vec<(AssetId, Vec<u8>)> = Vec::new();

        for chunk in &manifest.chunks {
            let start = data_start + chunk.offset as usize;
            let end = start + chunk.length as usize;
            if bundle.len() < end {
                return Err(BackendError::Packaging(format!(
                    "bundle truncated at chunk {}",
                    chunk.asset_id
                )));
            }
            let bytes = bundle[start..end].to_vec();

            // Verify SHA-256
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            let actual = hex::encode(hasher.finalize());
            if actual != chunk.sha256 {
                return Err(BackendError::Packaging(format!(
                    "sha256 mismatch for {}",
                    chunk.asset_id
                )));
            }
            assets.push((chunk.asset_id.clone(), bytes));
        }

        Ok((manifest, assets))
    }
}
