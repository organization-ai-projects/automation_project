//! projects/products/varina/backend/src/autopilot/handlers.rs
use common_json::to_value;
use protocol::{ApplyRequest, ApplyResponse, PreviewRequest, PreviewResponse};

use crate::automation::run_git_autopilot;
use crate::autopilot::{AutopilotMode, AutopilotPolicy};
use crate::git_github::suggest_policy_from_report;
use crate::handler_error::HandlerError;
use crate::pre_checks::PreChecks;
use crate::router::E_HANDLER_FAILED;

/// Preview = DryRun.
/// Does NOT make any random policy mutations.
/// Starts with AutopilotPolicy::default() (robust).
pub fn handle_preview_git_autopilot(req: PreviewRequest) -> Result<PreviewResponse, HandlerError> {
    let policy = AutopilotPolicy {
        fail_on_unrelated_changes: false, // Disabled for testing
        ..AutopilotPolicy::default()
    };
    let request_bytes = std::mem::size_of_val(&req);

    let report = match run_git_autopilot(AutopilotMode::DryRun, &policy) {
        Ok(r) => r,
        Err(e) => {
            return Err(HandlerError::internal_error(
                E_HANDLER_FAILED,
                format!("Autopilot execution failed: {e}"),
            ));
        }
    };

    let suggestion = suggest_policy_from_report(&report, &policy);
    // If your protocol does not have a suggestion field, you can either:
    // - ignore it (as done here),
    // - or add the notes to report.logs in run_git_autopilot (better place),
    // - or enrich the protocol later when ready.

    let payload = to_value(&report).map_err(|e| {
        HandlerError::internal_error(E_HANDLER_FAILED, format!("Failed to serialize report: {e}"))
    })?;
    Ok(PreviewResponse {
        summary: format!(
            "Preview successful (request_bytes={}, suggestion_notes={})",
            request_bytes,
            suggestion.notes.len()
        ),
        payload: Some(payload),
    })
}

/// Apply = ApplySafe.
/// Always uses the default policy (push disabled by default).
pub fn handle_apply_git_autopilot(req: ApplyRequest) -> Result<ApplyResponse, HandlerError> {
    let policy = AutopilotPolicy {
        fail_on_unrelated_changes: false,
        pre_checks: PreChecks::None,
        ..AutopilotPolicy::default()
    };

    let report = match run_git_autopilot(AutopilotMode::ApplySafe, &policy) {
        Ok(r) => r,
        Err(e) => {
            return Err(HandlerError::internal_error(
                E_HANDLER_FAILED,
                e.to_string(),
            ));
        }
    };

    let payload = to_value(&report).map_err(|e| {
        HandlerError::internal_error(E_HANDLER_FAILED, format!("Failed to serialize report: {e}"))
    })?;
    let request_bytes = std::mem::size_of_val(&req);
    Ok(ApplyResponse {
        result: format!("Application completed (request_bytes={request_bytes})"),
        payload: Some(payload),
    })
}
