use crate::diagnostics::error::AgentError;
use crate::verify::verify_step::VerifyStep;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyOutcome {
    pub step: VerifyStep,
    pub passed: bool,
    pub skipped: bool,
    pub output: Option<String>,
}

pub struct Verifier {
    pub allow_run: bool,
}

impl Verifier {
    pub fn new(allow_run: bool) -> Self {
        Self { allow_run }
    }

    /// Runs the given verification steps against `root`.
    ///
    /// When `allow_run` is `false` (safe default) every step is recorded as
    /// skipped without executing any external command.
    pub fn run(&self, root: &Path, steps: &[VerifyStep]) -> Result<Vec<VerifyOutcome>, AgentError> {
        let mut outcomes = Vec::new();
        for step in steps {
            if !self.allow_run {
                outcomes.push(VerifyOutcome {
                    step: step.clone(),
                    passed: false,
                    skipped: true,
                    output: None,
                });
                continue;
            }
            let output = std::process::Command::new(&step.command)
                .args(&step.args)
                .current_dir(root)
                .output()
                .map_err(|e| AgentError::Verify(format!("failed to run {}: {e}", step.command)))?;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            outcomes.push(VerifyOutcome {
                step: step.clone(),
                passed: output.status.success(),
                skipped: false,
                output: Some(stdout),
            });
        }
        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify::verify_step::VerifyStep;

    #[test]
    fn skips_steps_when_allow_run_false() {
        let steps = vec![VerifyStep::fmt(), VerifyStep::clippy()];
        let verifier = Verifier::new(false);
        let outcomes = verifier.run(std::path::Path::new("/tmp"), &steps).unwrap();
        assert_eq!(outcomes.len(), 2);
        assert!(outcomes.iter().all(|o| o.skipped));
        assert!(outcomes.iter().all(|o| !o.passed));
    }

    #[test]
    fn empty_steps_returns_empty_outcomes() {
        let verifier = Verifier::new(false);
        let outcomes = verifier.run(std::path::Path::new("/tmp"), &[]).unwrap();
        assert!(outcomes.is_empty());
    }
}
