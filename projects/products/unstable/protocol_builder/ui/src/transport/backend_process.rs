// projects/products/unstable/protocol_builder/ui/src/transport/backend_process.rs
use anyhow::Result;
use std::io::BufReader;
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct BackendProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub reader: BufReader<std::process::ChildStdout>,
}

impl BackendProcess {
    pub fn spawn(binary: &str) -> Result<Self> {
        let mut child = Command::new(binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = child.stdin.take().expect("stdin not captured");
        let stdout = child.stdout.take().expect("stdout not captured");
        let reader = BufReader::new(stdout);
        Ok(Self { child, stdin, reader })
    }
}
