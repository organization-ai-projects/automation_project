use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("job failure: job `{job}` exited with code {exit_code}")]
    JobFailure { job: String, exit_code: i32 },
    #[error("invalid config: {0}")]
    InvalidConfig(String),
    #[error("DAG error: {0}")]
    DagError(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl WorkflowError {
    /// Returns the stable exit code for this error variant.
    pub fn exit_code(&self) -> i32 {
        match self {
            WorkflowError::JobFailure { .. } => 1,
            WorkflowError::InvalidConfig(_) | WorkflowError::DagError(_) => 3,
            WorkflowError::Internal(_) => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_failure_exit_code() {
        let e = WorkflowError::JobFailure {
            job: "job1".to_string(),
            exit_code: 2,
        };
        assert_eq!(e.exit_code(), 1);
    }

    #[test]
    fn invalid_config_exit_code() {
        let e = WorkflowError::InvalidConfig("bad".to_string());
        assert_eq!(e.exit_code(), 3);
    }

    #[test]
    fn dag_error_exit_code() {
        let e = WorkflowError::DagError("cycle".to_string());
        assert_eq!(e.exit_code(), 3);
    }

    #[test]
    fn internal_error_exit_code() {
        let e = WorkflowError::Internal("oops".to_string());
        assert_eq!(e.exit_code(), 4);
    }
}
