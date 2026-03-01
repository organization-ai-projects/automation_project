// projects/products/unstable/digital_pet/ui/src/transport/backend_process.rs
use crate::diagnostics::error::AppError;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct BackendProcess {
    pub child: Child,
}

impl BackendProcess {
    pub fn spawn(scenario: &Path) -> Result<Self, AppError> {
        let child = Command::new("digital_pet_backend")
            .arg("serve")
            .arg("--scenario")
            .arg(scenario)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| AppError::Process(e.to_string()))?;
        Ok(Self { child })
    }
}

impl Drop for BackendProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
