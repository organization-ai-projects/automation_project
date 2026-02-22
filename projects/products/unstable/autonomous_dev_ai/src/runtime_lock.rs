use std::fs::{self, OpenOptions};
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// Runtime lock guard preventing concurrent autonomous runs on the same lock path.
pub struct RuntimeLockGuard {
    path: PathBuf,
    held: bool,
}

impl RuntimeLockGuard {
    pub fn acquire(path: impl Into<PathBuf>, run_id: &str) -> std::io::Result<Self> {
        let path = path.into();
        Self::try_acquire(&path, run_id).or_else(|err| {
            if err.kind() != ErrorKind::AlreadyExists {
                return Err(err);
            }

            if Self::is_active_owner(&path) {
                return Err(Error::new(
                    ErrorKind::WouldBlock,
                    format!(
                        "runtime lock is already held by another process: {}",
                        path.display()
                    ),
                ));
            }

            fs::remove_file(&path)?;
            Self::try_acquire(&path, run_id)
        })?;

        Ok(Self { path, held: true })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn try_acquire(path: &Path, run_id: &str) -> std::io::Result<()> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        writeln!(file, "pid={}", process::id())?;
        writeln!(file, "run_id={run_id}")?;
        writeln!(file, "created_at_unix={now}")?;
        file.flush()?;

        Ok(())
    }

    fn is_active_owner(path: &Path) -> bool {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return true,
        };

        let pid = content.lines().find_map(|line| {
            line.strip_prefix("pid=")
                .and_then(|value| value.trim().parse::<u32>().ok())
        });

        match pid {
            Some(pid) => process_exists(pid),
            None => true,
        }
    }
}

impl Drop for RuntimeLockGuard {
    fn drop(&mut self) {
        if self.held {
            let _ = fs::remove_file(&self.path);
            self.held = false;
        }
    }
}

#[cfg(target_os = "linux")]
fn process_exists(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

#[cfg(not(target_os = "linux"))]
fn process_exists(_pid: u32) -> bool {
    true
}
