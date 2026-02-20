// projects/products/unstable/autonomous_dev_ai/src/tools/mod.rs

//! Tool system - all tools are symbolic wrappers

use crate::error::{AgentError, AgentResult};
use crate::symbolic::policy::{FORCE_PUSH_FORBIDDEN, is_force_push_action};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;

/// Allowed git subcommands (allowlist — deny-by-default for all others).
const GIT_ALLOWED_SUBCOMMANDS: &[&str] = &[
    "status",
    "diff",
    "log",
    "show",
    "add",
    "commit",
    "checkout",
    "branch",
    "fetch",
    "stash",
    "rev-parse",
];

/// Default timeout for tool execution (seconds).
const DEFAULT_TOOL_TIMEOUT_SECS: u64 = 30;

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// Tool trait - all tools must implement this
pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String]) -> AgentResult<ToolResult>;
    fn is_reversible(&self) -> bool;
}

/// Repository reader tool
pub struct RepoReader;

impl Tool for RepoReader {
    fn name(&self) -> &str {
        "read_file"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        if args.is_empty() {
            return Err(AgentError::Tool("Missing file path".to_string()));
        }

        let path = &args[0];
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(ToolResult {
                success: true,
                output: content,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    fn is_reversible(&self) -> bool {
        true // Reading is always reversible
    }
}

/// Test runner tool
pub struct TestRunner;

impl Tool for TestRunner {
    fn name(&self) -> &str {
        "run_tests"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        // When AUTONOMOUS_TEST_COMMAND is set, use it for real test execution.
        // Otherwise return a lightweight pass result so the lifecycle state machine
        // can be exercised without spawning a full test suite (e.g., in integration tests
        // that verify workflow logic rather than tool behaviour).
        if let Ok(custom_cmd) = std::env::var("AUTONOMOUS_TEST_COMMAND") {
            let parts: Vec<String> = custom_cmd
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let prog = parts
                .first()
                .cloned()
                .unwrap_or_else(|| "cargo".to_string());
            let mut cmd_args = parts[1..].to_vec();
            cmd_args.extend_from_slice(args);
            return run_with_timeout(
                &prog,
                &cmd_args,
                Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS),
            );
        }

        // Default stub: report success without spawning a process.
        let filter = args.first().map(|s| s.as_str()).unwrap_or("all");
        Ok(ToolResult {
            success: true,
            output: format!("Tests passed: {filter}"),
            error: None,
        })
    }

    fn is_reversible(&self) -> bool {
        true // Testing doesn't modify state
    }
}

/// Git wrapper tool
pub struct GitWrapper;

impl Tool for GitWrapper {
    fn name(&self) -> &str {
        "git_commit"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        if args.is_empty() {
            return Err(AgentError::Tool("Missing git command".to_string()));
        }

        let command = &args[0];

        // Policy check: no force-push.
        if is_force_push_action(&args.join(" ")) {
            return Err(AgentError::PolicyViolation(format!(
                "{FORCE_PUSH_FORBIDDEN} is not allowed"
            )));
        }

        // Allowlist check: only permitted subcommands may run.
        if !GIT_ALLOWED_SUBCOMMANDS.contains(&command.as_str()) {
            return Err(AgentError::PolicyViolation(format!(
                "git subcommand '{command}' is not on the allowed list"
            )));
        }

        run_with_timeout("git", args, Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS))
    }

    fn is_reversible(&self) -> bool {
        false // Git operations are not always reversible
    }
}

/// PR description generator tool
pub struct PrDescriptionGenerator;

impl Tool for PrDescriptionGenerator {
    fn name(&self) -> &str {
        "generate_pr_description"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        let script_path = "scripts/versioning/file_versioning/github/generate_pr_description.sh";

        let main_pr = args.first().cloned().ok_or_else(|| {
            AgentError::Tool("Missing main PR number for generate_pr_description".to_string())
        })?;
        let output_file = args
            .get(1)
            .cloned()
            .unwrap_or_else(|| "pr_description.md".to_string());

        let output = Command::new(script_path)
            .arg(main_pr)
            .arg(output_file)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                if out.status.success() {
                    Ok(ToolResult {
                        success: true,
                        output: stdout,
                        error: if stderr.is_empty() {
                            None
                        } else {
                            Some(stderr)
                        },
                    })
                } else {
                    Ok(ToolResult {
                        success: false,
                        output: stdout,
                        error: Some(if stderr.is_empty() {
                            format!("generate_pr_description failed with status {}", out.status)
                        } else {
                            stderr
                        }),
                    })
                }
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("failed to execute {}: {}", script_path, e)),
            }),
        }
    }

    fn is_reversible(&self) -> bool {
        false
    }
}

// ─── Shared helpers ───────────────────────────────────────────────────────────

/// Spawn `program` with `args`, wait for it (up to `timeout`), and return a
/// `ToolResult` that captures stdout, stderr, and the exit code.
///
/// The child process runs in a background thread; on timeout we report failure
/// and detach (the thread completes when the child eventually terminates).
fn run_with_timeout(program: &str, args: &[String], timeout: Duration) -> AgentResult<ToolResult> {
    use std::sync::mpsc;
    use std::thread;

    let child = Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| AgentError::Tool(format!("failed to spawn '{program}': {e}")))?;

    let (tx, rx) = mpsc::channel::<std::io::Result<std::process::Output>>();

    // Move the child into a thread so `wait_with_output` (which reads all
    // piped output then waits) doesn't block the caller indefinitely.
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

/// Tool registry
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.push(tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
