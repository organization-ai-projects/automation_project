// projects/products/core/engine/src/bootstrap.rs
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use chrono::Utc;
use rand::RngCore;
use serde::{Deserialize, Serialize};

const CLAIM_FILE_NAME: &str = "owner.claim";
const CLAIM_USED_NAME: &str = "owner.claim.used";
const CLAIM_TTL_HOURS: i64 = 24;

#[derive(Debug, Clone)]
pub struct SetupState {
    pub setup_mode: bool,
    pub claim_path: PathBuf,
    pub used_marker_path: PathBuf,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerClaim {
    pub secret: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum BootstrapError {
    #[error("home directory is not available")]
    HomeDirUnavailable,

    #[error("claim file missing")]
    ClaimMissing,

    #[error("claim expired")]
    ClaimExpired,

    #[error("claim invalid")]
    ClaimInvalid,

    #[error("setup already completed")]
    SetupAlreadyCompleted,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(String),
}

pub fn ensure_owner_claim() -> Result<SetupState, BootstrapError> {
    let claim_path = owner_claim_path()?;
    let used_marker_path = owner_claim_used_path()?;

    if used_marker_path.exists() {
        if claim_path.exists() {
            let _ = fs::remove_file(&claim_path);
        }
        return Ok(SetupState {
            setup_mode: false,
            claim_path,
            used_marker_path,
            expires_at: None,
        });
    }

    if claim_path.exists() {
        let claim = read_claim(&claim_path)?;
        if claim.expires_at < Utc::now().timestamp() {
            let _ = fs::remove_file(&claim_path);
            let claim = create_claim(&claim_path)?;
            return Ok(SetupState {
                setup_mode: true,
                claim_path,
                used_marker_path,
                expires_at: Some(claim.expires_at),
            });
        }

        ensure_strict_permissions(&claim_path)?;
        return Ok(SetupState {
            setup_mode: true,
            claim_path,
            used_marker_path,
            expires_at: Some(claim.expires_at),
        });
    }

    let claim = create_claim(&claim_path)?;
    Ok(SetupState {
        setup_mode: true,
        claim_path,
        used_marker_path,
        expires_at: Some(claim.expires_at),
    })
}

pub fn validate_claim(secret: &str) -> Result<OwnerClaim, BootstrapError> {
    let claim_path = owner_claim_path()?;
    let used_marker_path = owner_claim_used_path()?;

    if used_marker_path.exists() {
        return Err(BootstrapError::SetupAlreadyCompleted);
    }

    if !claim_path.exists() {
        return Err(BootstrapError::ClaimMissing);
    }

    let claim = read_claim(&claim_path)?;
    if claim.expires_at < Utc::now().timestamp() {
        return Err(BootstrapError::ClaimExpired);
    }

    if claim.secret != secret {
        return Err(BootstrapError::ClaimInvalid);
    }

    Ok(claim)
}

pub fn consume_claim() -> Result<(), BootstrapError> {
    let claim_path = owner_claim_path()?;
    let used_marker_path = owner_claim_used_path()?;

    if let Some(parent) = used_marker_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(&used_marker_path)?;
    let timestamp = Utc::now().to_rfc3339();
    writeln!(file, "bootstrapped_at={timestamp}")?;

    if claim_path.exists() {
        let _ = fs::remove_file(&claim_path);
    }

    Ok(())
}

pub fn setup_complete() -> Result<bool, BootstrapError> {
    Ok(owner_claim_used_path()?.exists())
}

fn create_claim(path: &PathBuf) -> Result<OwnerClaim, BootstrapError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let secret = generate_secret();
    let created_at = Utc::now().timestamp();
    let expires_at = created_at + (CLAIM_TTL_HOURS * 3600);

    let claim = OwnerClaim {
        secret,
        created_at,
        expires_at,
    };

    write_claim(path, &claim)?;
    ensure_strict_permissions(path)?;
    Ok(claim)
}

fn write_claim(path: &PathBuf, claim: &OwnerClaim) -> Result<(), BootstrapError> {
    let data =
        common_json::to_bytes_pretty(claim).map_err(|e| BootstrapError::Json(e.to_string()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(path)?;
        file.write_all(&data)?;
        Ok(())
    }

    #[cfg(not(unix))]
    {
        let mut file = fs::File::create(path)?;
        file.write_all(&data)?;
        Ok(())
    }
}

fn read_claim(path: &PathBuf) -> Result<OwnerClaim, BootstrapError> {
    let data = fs::read(path)?;
    let claim = common_json::from_slice(&data).map_err(|e| BootstrapError::Json(e.to_string()))?;
    Ok(claim)
}

fn generate_secret() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut bytes);
    bytes_to_hex(&bytes)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(nibble_to_hex(byte >> 4));
        out.push(nibble_to_hex(byte & 0x0f));
    }
    out
}

fn nibble_to_hex(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => '0',
    }
}

fn owner_claim_dir() -> Result<PathBuf, BootstrapError> {
    if let Ok(dir) = std::env::var("ENGINE_OWNER_CLAIM_DIR")
        && !dir.trim().is_empty()
    {
        return Ok(PathBuf::from(dir));
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .map_err(|_| BootstrapError::HomeDirUnavailable)?;
    Ok(home.join(".automation_project"))
}

fn owner_claim_path() -> Result<PathBuf, BootstrapError> {
    Ok(owner_claim_dir()?.join(CLAIM_FILE_NAME))
}

fn owner_claim_used_path() -> Result<PathBuf, BootstrapError> {
    Ok(owner_claim_dir()?.join(CLAIM_USED_NAME))
}

fn ensure_strict_permissions(path: &PathBuf) -> Result<(), BootstrapError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)?;
        let mut perms = metadata.permissions();
        if perms.mode() & 0o077 != 0 {
            perms.set_mode(0o600);
            fs::set_permissions(path, perms)?;
        }
    }
    Ok(())
}
