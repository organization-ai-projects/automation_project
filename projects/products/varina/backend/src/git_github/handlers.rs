use crate::automation::run_git_autopilot;
use crate::autopilot::{AutopilotMode, AutopilotPolicy, AutopilotReport};

pub struct PreviewRequest {
    pub policy_overrides: Option<AutopilotPolicy>,
}

pub struct PreviewResponse {
    pub report: AutopilotReport,
    pub suggestion: PolicySuggestion, // Ajout des suggestions IA dans la réponse
}

pub struct ApplyRequest {
    pub policy_overrides: Option<AutopilotPolicy>,
}

pub struct ApplyResponse {
    pub report: AutopilotReport,
}

pub struct PolicySuggestion {
    pub allow_push: Option<bool>,
    pub fail_on_unrelated_changes: Option<bool>,
    pub notes: Vec<String>,
}

pub fn suggest_policy_from_report(
    report: &AutopilotReport,
    policy: &AutopilotPolicy,
) -> PolicySuggestion {
    let mut notes = Vec::new();

    if !report.classified.blocked.is_empty() && policy.fail_on_unrelated_changes {
        notes.push("Des changements bloqués ont été détectés. Désactivez fail_on_unrelated_changes pour continuer.".to_string());
    }

    PolicySuggestion {
        allow_push: None, // Exemple : aucune suggestion explicite pour allow_push
        fail_on_unrelated_changes: if !report.classified.blocked.is_empty() {
            Some(false)
        } else {
            None
        },
        notes,
    }
}

pub fn handle_preview_git_autopilot(req: PreviewRequest) -> Result<PreviewResponse, String> {
    let policy = req.policy_overrides.unwrap_or_default();
    let report = run_git_autopilot(AutopilotMode::DryRun, &policy)?;
    let suggestion = suggest_policy_from_report(&report, &policy);

    // Ajout des logs structurés dans le rapport
    let mut logs = report.logs.clone();
    logs.push("Prévisualisation terminée avec succès".to_string());

    Ok(PreviewResponse { report, suggestion })
}

pub fn handle_apply_git_autopilot(req: ApplyRequest) -> Result<ApplyResponse, String> {
    let policy = req.policy_overrides.unwrap_or_default();
    let report = run_git_autopilot(AutopilotMode::ApplySafe, &policy)?;

    // Ajout des logs structurés dans le rapport
    let mut logs = report.logs.clone();
    logs.push("Application terminée avec succès".to_string());

    Ok(ApplyResponse { report })
}
