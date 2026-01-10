// projects/products/varina/backend/src/git_github/policy_suggestions.rs
use crate::autopilot::{AutopilotPolicy, AutopilotReport};

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

    // 1) blocked => on n’essaie JAMAIS de "passer quand même"
    if !report.classified.blocked.is_empty() {
        notes.push(
            "Changements bloqués détectés: retire/déplace ces fichiers (ou ajuste blocked_prefixes si c'est un faux positif)."
                .to_string(),
        );
    }

    // 2) unrelated => si la policy refuse, on propose une action humaine, pas une bidouille auto
    if policy.fail_on_unrelated_changes && !report.classified.unrelated.is_empty() {
        notes.push(
            "Changements non liés détectés: commit séparé recommandé (l'autopilot refuse par défaut pour éviter les splits magiques)."
                .to_string(),
        );
    }

    // 3) push => on ne suggère pas d'activer push automatiquement
    if report.plan.will_push && !policy.allow_push {
        notes.push(
            "Le plan inclut un push mais la policy l'interdit (allow_push=false).".to_string(),
        );
    }

    PolicySuggestion {
        allow_push: None,
        fail_on_unrelated_changes: None,
        notes,
    }
}
