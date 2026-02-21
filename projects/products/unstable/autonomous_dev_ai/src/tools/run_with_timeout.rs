// projects/products/unstable/autonomous_dev_ai/src/tools/run_with_timeout.rs
use super::ToolResult;
use crate::error::{AgentError, AgentResult};
use std::process::Command;
use std::time::Duration;

/// Spawn `program` with `args`, wait for it (up to `timeout`), and return a
/// `ToolResult` capturing stdout, stderr, and exit status.
pub(crate) fn run_with_timeout(
    program: &str,
    args: &[String],
    timeout: Duration,
) -> AgentResult<ToolResult> {
    use std::sync::mpsc;
    use std::thread;

    let child = Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| AgentError::Tool(format!("failed to spawn '{program}': {e}")))?;

    let (tx, rx) = mpsc::channel::<std::io::Result<std::process::Output>>();

    let _handle = thread::spawn(move || {
        let _ = tx.send(child.wait_with_output());
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            Ok(ToolResult {
                success: output.status.success(),
                output: stdout,
                error: if stderr.is_empty() {
                    None
                } else {
                    Some(stderr)
                },
            })
        }
        Ok(Err(e)) => Err(AgentError::Tool(format!("wait_with_output error: {e}"))),
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(ToolResult {
            success: false,
            output: String::new(),
            error: Some(format!(
                "'{program}' timed out after {}s",
                timeout.as_secs()
            )),
        }),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(AgentError::Tool(
            "unexpected channel disconnect".to_string(),
        )),
    }
}
