//projects/products/core/central_ui/src/claims.rs
use std::path::PathBuf;

use crate::owner_claim::OwnerClaim;

pub(crate) fn owner_claim_path(claim_dir: Option<String>) -> PathBuf {
    if let Some(dir) = claim_dir {
        return PathBuf::from(dir).join("owner.claim");
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".automation_project")
        .join("owner.claim")
}

pub(crate) fn read_claim(path: &PathBuf) -> Result<OwnerClaim, std::io::Error> {
    let data = std::fs::read(path)?;
    let claim: OwnerClaim = common_json::from_slice(&data).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid owner claim {}: {}", path.display(), e),
        )
    })?;
    Ok(claim)
}
