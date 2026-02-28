use anyhow::{Result, Context};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl BackendProcess {
    pub fn spawn() -> Result<Self> {
        let mut child = Command::new("meta_determinism_guard_backend")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn backend process")?;

        let stdin = child.stdin.take().context("No stdin")?;
        let stdout = BufReader::new(child.stdout.take().context("No stdout")?);

        Ok(Self { _child: child, stdin, stdout })
    }

    pub fn send_line(&mut self, line: &str) -> Result<()> {
        writeln!(self.stdin, "{}", line)?;
        self.stdin.flush()?;
        Ok(())
    }

    pub fn recv_line(&mut self) -> Result<String> {
        let mut line = String::new();
        self.stdout.read_line(&mut line)?;
        Ok(line.trim().to_string())
    }
}
