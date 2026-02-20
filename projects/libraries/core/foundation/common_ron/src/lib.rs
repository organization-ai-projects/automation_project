use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RonIoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON deserialize error: {0}")]
    Deserialize(String),
    #[error("RON serialize error: {0}")]
    Serialize(String),
    #[error("Invalid target path for safe write: {0}")]
    InvalidPath(String),
}

pub type RonIoResult<T> = Result<T, RonIoError>;

pub fn read_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> RonIoResult<T> {
    let content = fs::read_to_string(path)?;
    ron::from_str(&content).map_err(|e| RonIoError::Deserialize(e.to_string()))
}

pub fn read_ron_str<T: DeserializeOwned>(input: &str) -> RonIoResult<T> {
    ron::from_str(input).map_err(|e| RonIoError::Deserialize(e.to_string()))
}

pub fn write_ron<T: Serialize>(path: impl AsRef<Path>, value: &T) -> RonIoResult<()> {
    let target = path.as_ref();
    let parent = target.parent().ok_or_else(|| {
        RonIoError::InvalidPath(format!("missing parent directory for {}", target.display()))
    })?;
    let file_name = target.file_name().ok_or_else(|| {
        RonIoError::InvalidPath(format!("missing file name for {}", target.display()))
    })?;

    let content = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())
        .map_err(|e| RonIoError::Serialize(e.to_string()))?;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let temp_name = format!(
        ".{}.tmp-{}-{}",
        file_name.to_string_lossy(),
        std::process::id(),
        nanos
    );
    let temp_path = parent.join(temp_name);

    {
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
    }

    match fs::rename(&temp_path, target) {
        Ok(()) => Ok(()),
        Err(e) => {
            let _ = fs::remove_file(&temp_path);
            Err(RonIoError::Io(e))
        }
    }
}
