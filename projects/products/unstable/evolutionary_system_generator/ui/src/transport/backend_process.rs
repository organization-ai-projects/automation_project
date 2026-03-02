#[allow(dead_code)]
pub struct BackendProcess {
    child: std::process::Child,
}

#[allow(dead_code)]
impl BackendProcess {
    pub fn spawn(binary_path: &str) -> std::io::Result<Self> {
        use std::process::{Command, Stdio};
        let child = Command::new(binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self { child })
    }

    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }
}
