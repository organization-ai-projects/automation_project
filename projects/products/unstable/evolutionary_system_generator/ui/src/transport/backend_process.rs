// projects/products/unstable/evolutionary_system_generator/ui/src/transport/backend_process.rs
use std::io;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    child: Child,
}

impl BackendProcess {
    pub fn spawn(binary_path: &str) -> io::Result<Self> {
        let primary = Self::spawn_command(binary_path, &[]);
        if let Ok(child) = primary {
            return Ok(Self { child });
        }

        // Fallback: workspace execution without requiring `evo-backend` in PATH.
        let fallback = Self::spawn_command(
            "cargo",
            &[
                "run",
                "-q",
                "-p",
                "evolutionary_system_generator_backend",
                "--",
            ],
        );
        match fallback {
            Ok(child) => Ok(Self { child }),
            Err(fallback_err) => {
                let primary_err = primary
                    .err()
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "unknown error".to_string());
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "failed to spawn backend '{binary_path}' ({primary_err}); fallback via cargo failed ({fallback_err})"
                    ),
                ))
            }
        }
    }

    fn spawn_command(program: &str, args: &[&str]) -> io::Result<Child> {
        Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
    }

    pub fn take_stdin(&mut self) -> io::Result<ChildStdin> {
        self.child
            .stdin
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "backend stdin unavailable"))
    }

    pub fn take_stdout(&mut self) -> io::Result<ChildStdout> {
        self.child
            .stdout
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "backend stdout unavailable"))
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.child.kill()
    }
}
