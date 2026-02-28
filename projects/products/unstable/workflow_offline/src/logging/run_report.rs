use crate::exec::exec_result::ExecResult;
use serde::{Deserialize, Serialize};

/// A snapshot of a single job execution within a run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobReport {
    /// The job's string identifier from the workflow config.
    pub job_id: String,
    /// Exit code returned by the command.
    pub exit_code: i32,
    /// Captured standard output.
    pub stdout: String,
    /// Captured standard error.
    pub stderr: String,
}

impl JobReport {
    pub fn new(job_id: impl Into<String>, result: ExecResult) -> Self {
        Self {
            job_id: job_id.into(),
            exit_code: result.exit_code,
            stdout: result.stdout,
            stderr: result.stderr,
        }
    }

    /// Returns `true` if this job succeeded (exit code 0).
    #[allow(dead_code)]
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

/// A structured report of a complete workflow run.
///
/// This is the primary output emitted by `WorkflowEngine::run`.
/// When `--json` is specified, the entire `RunReport` is serialised
/// to stdout.  It also embeds the serialised `EventLog` for replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    /// Name of the executed workflow.
    pub workflow_name: String,
    /// Seed used for deterministic ordering.
    pub seed: u64,
    /// Per-job execution reports in execution order.
    pub jobs: Vec<JobReport>,
    /// `true` if every job exited with code 0.
    pub success: bool,
    /// Serialised runtime_core `EventLog` (JSON bytes, base64-encoded
    /// for embedding in JSON output).
    pub event_log_json: String,
}

impl RunReport {
    /// Returns the jobs in the order they were executed.
    #[allow(dead_code)]
    pub fn execution_order(&self) -> Vec<&str> {
        self.jobs.iter().map(|j| j.job_id.as_str()).collect()
    }
}
