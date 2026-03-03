use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl BackendProcess {
    pub fn spawn() -> Result<Self> {
        let mut child = spawn_backend_process()?;

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

fn spawn_backend_process() -> Result<Child> {
    if let Some(workspace_root) = find_workspace_root(std::env::current_dir().ok().as_deref()) {
        if let Ok(child) = spawn_backend_with_cargo_run(&workspace_root) {
            return Ok(child);
        }

        if let Ok(child) = spawn_backend_by_command("repo_contract_enforcer_backend") {
            return Ok(child);
        }

        let binary_path = workspace_root
            .join("target")
            .join("debug")
            .join(executable_name("repo_contract_enforcer_backend"));

        if binary_path.is_file()
            && let Ok(child) = spawn_backend_by_command(binary_path.as_os_str())
        {
            return Ok(child);
        }

        let build_status = Command::new("cargo")
            .arg("build")
            .arg("-q")
            .arg("-p")
            .arg("repo_contract_enforcer_backend")
            .current_dir(&workspace_root)
            .status()
            .context("failed to run cargo build for repo_contract_enforcer_backend")?;

        if build_status.success()
            && binary_path.is_file()
            && let Ok(child) = spawn_backend_by_command(binary_path.as_os_str())
        {
            return Ok(child);
        }
    }

    if let Ok(child) = spawn_backend_by_command("repo_contract_enforcer_backend") {
        return Ok(child);
    }

    anyhow::bail!(
        "failed to spawn repo_contract_enforcer_backend (tried PATH, target/debug, and cargo build fallback)"
    );
}

fn spawn_backend_with_cargo_run(workspace_root: &Path) -> Result<Child> {
    Command::new("cargo")
        .arg("run")
        .arg("-q")
        .arg("-p")
        .arg("repo_contract_enforcer_backend")
        .arg("--")
        .arg("serve")
        .current_dir(workspace_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("backend cargo run spawn failed")
}

fn spawn_backend_by_command<S: AsRef<std::ffi::OsStr>>(cmd: S) -> Result<Child> {
    Command::new(cmd)
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("backend spawn failed")
}

fn find_workspace_root(start: Option<&Path>) -> Option<PathBuf> {
    let mut cursor = start?.to_path_buf();
    loop {
        if cursor.join("Cargo.toml").is_file() && cursor.join("projects").is_dir() {
            return Some(cursor);
        }
        if !cursor.pop() {
            break;
        }
    }
    None
}

fn executable_name(base: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        format!("{base}.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        base.to_string()
    }
}
