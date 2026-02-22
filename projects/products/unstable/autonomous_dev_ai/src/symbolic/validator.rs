// projects/products/unstable/autonomous_dev_ai/src/symbolic/validator.rs
use crate::error::AgentResult;

/// Validator for symbolic layer
pub struct Validator {
    pub strict: bool,
}

impl Validator {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    pub fn validate_plan_step(&self, tool: &str, args: &[String]) -> AgentResult<bool> {
        if tool.trim().is_empty() {
            return Ok(false);
        }

        if tool == "run_tests" {
            // In strict mode we expect an explicit runtime command.
            if self.strict && args.is_empty() {
                return Ok(false);
            }

            // When args are present, ensure cargo subcommands are from an allowlist.
            if args.first().map(|s| s.as_str()) == Some("cargo")
                && let Some(subcommand) = args.get(1)
                && self.strict
                && !matches!(subcommand.as_str(), "check" | "test" | "clippy")
            {
                return Ok(false);
            }
        }

        if tool == "read_file" && self.strict && args.is_empty() {
            return Ok(false);
        }

        Ok(true)
    }
}
