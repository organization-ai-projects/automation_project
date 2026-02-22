// projects/products/unstable/auto_manager_ai/src/adapters/ci_adapter.rs

use super::ci_context::CiContext;
use super::error::AdapterError;

/// CI adapter producing normalized environment-based signals.
#[derive(Debug)]
pub struct CiAdapter;

impl CiAdapter {
    /// Create a new CI adapter
    pub fn new() -> Self {
        Self
    }

    /// Get CI context from standard CI variables.
    pub fn get_context(&self) -> Result<CiContext, AdapterError> {
        let ci_flag = std::env::var("CI").unwrap_or_default();
        let run_id = std::env::var("GITHUB_RUN_ID").ok();
        let workflow = std::env::var("GITHUB_WORKFLOW").ok();
        let job = std::env::var("GITHUB_JOB").ok();
        let provider = if std::env::var("GITHUB_ACTIONS").ok().as_deref() == Some("true") {
            "github_actions".to_string()
        } else if ci_flag.eq_ignore_ascii_case("true") {
            "generic_ci".to_string()
        } else {
            "none".to_string()
        };

        if provider == "none" {
            return Ok(CiContext {
                available: false,
                status: "ci signal unavailable".to_string(),
                provider,
                run_id: None,
                workflow: None,
                job: None,
                degraded: true,
                error_code: Some("CI_ADAPTER_SIGNAL_UNAVAILABLE".to_string()),
            });
        }

        let degraded = run_id.is_none();
        Ok(CiContext {
            available: true,
            status: if degraded {
                "ci provider detected but run_id missing".to_string()
            } else {
                "ci signal available".to_string()
            },
            provider,
            run_id,
            workflow,
            job,
            degraded,
            error_code: degraded.then(|| "CI_ADAPTER_RUN_ID_MISSING".to_string()),
        })
    }
}

impl Default for CiAdapter {
    fn default() -> Self {
        Self::new()
    }
}
