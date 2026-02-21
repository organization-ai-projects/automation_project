// projects/products/unstable/autonomous_dev_ai/src/tools/run_with_timeout.rs
use super::ToolResult;
use crate::error::{AgentError, AgentResult};
use std::process::Command;
use std::time::Duration;
use std::time::Instant;

/// Spawn `program` with `args`, wait for it (up to `timeout`), and return a
/// `ToolResult` capturing stdout, stderr, and exit status.
pub(crate) fn run_with_timeout(
    program: &str,
    args: &[String],
    timeout: Duration,
) -> AgentResult<ToolResult> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| AgentError::Tool(format!("failed to spawn '{program}': {e}")))?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                let output = child
                    .wait_with_output()
                    .map_err(|e| AgentError::Tool(format!("wait_with_output error: {e}")))?;
                return Ok(build_tool_result(output));
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    if let Err(e) = child.kill() {
                        return Err(AgentError::Tool(format!(
                            "failed to terminate '{program}' after timeout: {e}"
                        )));
                    }
                    let output = child.wait_with_output().map_err(|e| {
                        AgentError::Tool(format!("wait_with_output after kill error: {e}"))
                    })?;
                    let mut result = build_tool_result(output);
                    result.success = false;
                    result.error = Some(format!(
                        "'{program}' timed out after {}s and was terminated",
                        timeout.as_secs()
                    ));
                    return Ok(result);
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(e) => return Err(AgentError::Tool(format!("try_wait error: {e}"))),
        }
    }
}

fn build_tool_result(output: std::process::Output) -> ToolResult {
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let status = output.status;
    let success = status.success();
    let code = status
        .code()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "signal".to_string());
    let error = if success {
        if stderr.is_empty() {
            None
        } else {
            Some(stderr)
        }
    } else if stderr.is_empty() {
        Some(format!("process exited with code {code}"))
    } else {
        Some(format!("process exited with code {code}: {stderr}"))
    };

    ToolResult {
        success,
        output: stdout,
        error,
    }
}
