use crate::config::job_config::JobConfig;
use crate::diagnostics::error::WorkflowError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

/// Top-level workflow definition loaded from a TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Human-readable name for this workflow.
    pub name: String,
    /// Ordered list of job definitions.
    pub jobs: Vec<JobConfig>,
}

impl WorkflowConfig {
    /// Loads and validates a `WorkflowConfig` from a TOML file on disk.
    pub fn from_file(path: &Path) -> Result<Self, WorkflowError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            WorkflowError::InvalidConfig(format!("cannot read `{}`: {e}", path.display()))
        })?;
        let config: WorkflowConfig = toml::from_str(&content).map_err(|e| {
            WorkflowError::InvalidConfig(format!("TOML parse error in `{}`: {e}", path.display()))
        })?;
        config.validate()?;
        Ok(config)
    }

    /// Validates the config: checks for duplicate IDs and missing dependency references.
    pub fn validate(&self) -> Result<(), WorkflowError> {
        if self.name.trim().is_empty() {
            return Err(WorkflowError::InvalidConfig(
                "workflow name must not be empty".to_string(),
            ));
        }
        let ids: HashSet<&str> = self.jobs.iter().map(|j| j.id.as_str()).collect();
        if ids.len() != self.jobs.len() {
            return Err(WorkflowError::InvalidConfig(
                "duplicate job IDs detected".to_string(),
            ));
        }
        for job in &self.jobs {
            for dep in &job.deps {
                if !ids.contains(dep.as_str()) {
                    return Err(WorkflowError::DagError(format!(
                        "job `{}` depends on unknown job `{dep}`",
                        job.id
                    )));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(jobs: Vec<JobConfig>) -> WorkflowConfig {
        WorkflowConfig {
            name: "test".to_string(),
            jobs,
        }
    }

    fn job(id: &str, deps: Vec<&str>) -> JobConfig {
        JobConfig {
            id: id.to_string(),
            command: "echo".to_string(),
            args: vec![],
            deps: deps.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn valid_config_passes() {
        let cfg = make_config(vec![job("a", vec![]), job("b", vec!["a"])]);
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let cfg = make_config(vec![job("a", vec![]), job("a", vec![])]);
        assert!(matches!(cfg.validate(), Err(WorkflowError::InvalidConfig(_))));
    }

    #[test]
    fn missing_dep_is_rejected() {
        let cfg = make_config(vec![job("b", vec!["nonexistent"])]);
        assert!(matches!(cfg.validate(), Err(WorkflowError::DagError(_))));
    }

    #[test]
    fn empty_name_is_rejected() {
        let cfg = WorkflowConfig {
            name: "  ".to_string(),
            jobs: vec![],
        };
        assert!(matches!(cfg.validate(), Err(WorkflowError::InvalidConfig(_))));
    }
}
