// projects/products/unstable/auto_manager_ai/src/adapters/gh_adapter.rs

use std::process::Command;

use common_json::{JsonAccess, from_str};

use super::error::AdapterError;
use super::gh_context::GhContext;

/// GitHub adapter using structured gh CLI responses.
#[derive(Debug)]
pub struct GhAdapter;

impl GhAdapter {
    /// Create a new GitHub adapter
    pub fn new() -> Self {
        Self
    }

    /// Get GitHub context from gh JSON commands.
    pub fn get_context(&self) -> Result<GhContext, AdapterError> {
        let repo_view = Command::new("gh")
            .args(["repo", "view", "--json", "nameWithOwner,defaultBranchRef"])
            .output()
            .map_err(|e| {
                AdapterError::non_retryable(
                    "GH_ADAPTER_COMMAND_UNAVAILABLE",
                    format!("failed to spawn gh: {e}"),
                )
            })?;

        if !repo_view.status.success() {
            return Err(AdapterError::retryable(
                "GH_ADAPTER_REPO_VIEW_FAILED",
                String::from_utf8_lossy(&repo_view.stderr).into_owned(),
            ));
        }

        let repo_json = String::from_utf8(repo_view.stdout).map_err(|e| {
            AdapterError::non_retryable(
                "GH_ADAPTER_REPO_UTF8_INVALID",
                format!("invalid utf8 from gh repo view: {e}"),
            )
        })?;
        let repo_payload: common_json::Json = from_str(&repo_json).map_err(|e| {
            AdapterError::non_retryable(
                "GH_ADAPTER_REPO_JSON_INVALID",
                format!("invalid json from gh repo view: {e}"),
            )
        })?;

        let repo = repo_payload
            .get_field("nameWithOwner")
            .ok()
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);
        let default_branch = repo_payload
            .get_field("defaultBranchRef")
            .ok()
            .and_then(|v| v.get_field("name").ok())
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);

        let pr_list = Command::new("gh")
            .args([
                "pr", "list", "--state", "open", "--json", "number", "--limit", "100",
            ])
            .output()
            .map_err(|e| {
                AdapterError::retryable("GH_ADAPTER_PR_LIST_FAILED", format!("gh pr list: {e}"))
            })?;

        let (open_pr_count, degraded, error_code, status) = if pr_list.status.success() {
            let pr_json = String::from_utf8(pr_list.stdout).map_err(|e| {
                AdapterError::non_retryable(
                    "GH_ADAPTER_PR_UTF8_INVALID",
                    format!("invalid utf8 from gh pr list: {e}"),
                )
            })?;
            let pr_payload: common_json::Json = from_str(&pr_json).map_err(|e| {
                AdapterError::non_retryable(
                    "GH_ADAPTER_PR_JSON_INVALID",
                    format!("invalid json from gh pr list: {e}"),
                )
            })?;
            let count = pr_payload.as_array().map(|v| v.len()).unwrap_or_default();
            (Some(count), false, None, "gh context available".to_string())
        } else {
            (
                None,
                true,
                Some("GH_ADAPTER_PR_LIST_DEGRADED".to_string()),
                "gh repo context available, PR listing degraded".to_string(),
            )
        };

        Ok(GhContext {
            available: true,
            status,
            repo,
            default_branch,
            open_pr_count,
            degraded,
            error_code,
        })
    }
}

impl Default for GhAdapter {
    fn default() -> Self {
        Self::new()
    }
}
