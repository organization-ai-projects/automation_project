//! projects/products/varina/backend/src/autopilot/handlers.rs
use common_json::to_value;
use protocol::{ApplyRequest, ApplyResponse, PreviewRequest, PreviewResponse};

use crate::automation::run_git_autopilot;
use crate::autopilot::{AutopilotMode, AutopilotPolicy};
use crate::git_github::suggest_policy_from_report;
use crate::pre_checks::PreChecks;

/// Preview = DryRun.
/// Does NOT make any random policy mutations.
/// Starts with AutopilotPolicy::default() (robust).
pub fn handle_preview_git_autopilot(_req: PreviewRequest) -> Result<PreviewResponse, String> {
    let policy = AutopilotPolicy {
        fail_on_unrelated_changes: false, // Disabled for testing
        ..AutopilotPolicy::default()
    };

    println!(
        "[debug] handle_preview_git_autopilot: Starting with policy: {:?}",
        policy
    );

    let report = match run_git_autopilot(AutopilotMode::DryRun, &policy) {
        Ok(r) => r,
        Err(e) => {
            println!("[error] handle_preview_git_autopilot: Error running autopilot: {e}");
            return Err(format!("Autopilot execution failed: {e}")); // More precise error message
        }
    };

    let _suggestion = suggest_policy_from_report(&report, &policy);
    // If your protocol does not have a suggestion field, you can either:
    // - ignore it (as done here),
    // - or add the notes to report.logs in run_git_autopilot (better place),
    // - or enrich the protocol later when ready.

    Ok(PreviewResponse {
        summary: "Preview successful".to_string(),
        payload: Some(to_value(&report).expect("serialize report")),
    })
}

/// Apply = ApplySafe.
/// Always uses the default policy (push disabled by default).
pub fn handle_apply_git_autopilot(_req: ApplyRequest) -> Result<ApplyResponse, String> {
    let policy = AutopilotPolicy {
        fail_on_unrelated_changes: false,
        pre_checks: PreChecks::None,
        ..AutopilotPolicy::default()
    };

    println!(
        "[debug] handle_apply_git_autopilot: Starting with policy: {:?}",
        policy
    );

    let report = match run_git_autopilot(AutopilotMode::ApplySafe, &policy) {
        Ok(r) => r,
        Err(e) => {
            println!("[error] handle_apply_git_autopilot: Error running autopilot: {e}");
            return Err(e.to_string());
        }
    };

    Ok(ApplyResponse {
        result: "Application completed".to_string(),
        payload: Some(to_value(&report).expect("serialize report")),
    })
}
