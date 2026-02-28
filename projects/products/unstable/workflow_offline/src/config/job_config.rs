use serde::{Deserialize, Serialize};

/// Configuration for a single job within a workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfig {
    /// Unique identifier for this job within the workflow.
    pub id: String,
    /// The command to execute.
    pub command: String,
    /// Arguments to pass to the command.
    #[serde(default)]
    pub args: Vec<String>,
    /// IDs of jobs that must complete before this one runs.
    #[serde(default)]
    pub deps: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_args_and_deps() {
        let toml_str = r#"id = "job1"
command = "echo"
"#;
        let job: JobConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(job.id, "job1");
        assert_eq!(job.command, "echo");
        assert!(job.args.is_empty());
        assert!(job.deps.is_empty());
    }

    #[test]
    fn with_args_and_deps() {
        let toml_str = r#"id = "job2"
command = "echo"
args = ["hello"]
deps = ["job1"]
"#;
        let job: JobConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(job.id, "job2");
        assert_eq!(job.args, vec!["hello"]);
        assert_eq!(job.deps, vec!["job1"]);
    }
}
