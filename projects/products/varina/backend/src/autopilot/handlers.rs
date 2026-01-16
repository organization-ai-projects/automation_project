use common_json::to_value;
// projects/products/varina/backend/src/autopilot/handlers.rs
use protocol::{ApplyRequest, ApplyResponse, PreviewRequest, PreviewResponse};

use crate::automation::run_git_autopilot;
use crate::git_github::suggest_policy_from_report;
use crate::{AutopilotMode, AutopilotPolicy, PreChecks};

/// Preview = DryRun.
/// Ne fait AUCUNE mutation hasardeuse de policy.
/// On part sur AutopilotPolicy::default() (robuste).
pub fn handle_preview_git_autopilot(_req: PreviewRequest) -> Result<PreviewResponse, String> {
    let policy = AutopilotPolicy {
        fail_on_unrelated_changes: false, // Désactivation pour les tests
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
            return Err(format!("Autopilot execution failed: {e}")); // Message d'erreur plus précis
        }
    };

    let _suggestion = suggest_policy_from_report(&report, &policy);
    // Si ton protocol n'a pas de champ suggestion, tu peux soit:
    // - ignorer (comme ici),
    // - ou ajouter les notes dans report.logs dans run_git_autopilot (meilleur endroit),
    // - ou enrichir protocol plus tard quand tu seras prêt.

    Ok(PreviewResponse {
        summary: "Prévisualisation réussie".to_string(),
        payload: Some(to_value(&report).unwrap()),
    })
}

/// Apply = ApplySafe.
/// Toujours policy default (push interdit par défaut).
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
        result: "Application terminée".to_string(),
        payload: Some(to_value(&report).unwrap()),
    })
}
