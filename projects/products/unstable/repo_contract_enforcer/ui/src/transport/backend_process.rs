use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl BackendProcess {
    pub fn spawn() -> Result<Self> {
        let mut child = Command::new("repo_contract_enforcer_backend")
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("failed to spawn repo_contract_enforcer_backend")?;

        let stdin = child.stdin.take().context("backend stdin unavailable")?;
        let stdout = child.stdout.take().context("backend stdout unavailable")?;

        Ok(Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        })
    }

    pub fn send_line(&mut self, line: &str) -> Result<()> {
        writeln!(&mut self.stdin, "{line}")?;
        self.stdin.flush()?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut line = String::new();
        let n = self.stdout.read_line(&mut line)?;
        if n == 0 {
            anyhow::bail!("backend closed stdout")
        }
        Ok(line)
    }

    pub fn shutdown(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
