// projects/products/varina/backend/src/autopilot/autopilot_policy.rs
use serde::{Deserialize, Serialize};

use crate::pre_checks::PreChecks;

/// Policy de sécurité pour l’autopilot.
/// Idée: le code "IA" ne décide pas au feeling, il applique une policy déterministe.
/// Définit les règles de sécurité et les politiques appliquées par l'autopilot.
/// Ces règles déterminent les fichiers pertinents, les branches protégées, et les actions autorisées.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopilotPolicy {
    /// Branches interdites de commit direct.
    pub protected_branches: Vec<String>,

    /// Pré-checks à exécuter avant d'agir.
    pub pre_checks: PreChecks,

    /// Ce que tu considères "pertinent" (allowlist).
    /// Exemple: ["src/", "tests/", "crates/"]
    pub relevant_prefixes: Vec<String>,

    /// Fichiers exacts considérés pertinents même hors prefixes.
    /// Exemple: ["Cargo.toml", "Cargo.lock", "README.md"]
    pub relevant_files: Vec<String>,

    /// Tout ce qui matche ici est refusé (même si pertinent).
    /// Exemple: ["target/", ".env", ".automation_project/secrets"]
    pub blocked_prefixes: Vec<String>,

    /// Si true: si des fichiers non-pertinents existent, on refuse d'agir.
    /// (recommandé pour éviter split/branch auto "magique".)
    pub fail_on_unrelated_changes: bool,

    /// Autoriser le push automatique.
    pub allow_push: bool,

    /// Remote à utiliser si push autorisé. (ex: "origin")
    pub push_remote: String,

    /// Si push autorisé, pousser la branche courante en upstream si nécessaire.
    pub push_set_upstream_if_missing: bool,
}

impl Default for AutopilotPolicy {
    fn default() -> Self {
        Self {
            protected_branches: vec!["main".into(), "dev".into()],
            pre_checks: PreChecks::FmtCheckAndTests,
            relevant_prefixes: vec!["src/".into(), "tests/".into()],
            relevant_files: vec!["Cargo.toml".into(), "Cargo.lock".into()],
            blocked_prefixes: vec!["target/".into(), ".env".into()],
            fail_on_unrelated_changes: true,
            allow_push: false,
            push_remote: "origin".into(),
            push_set_upstream_if_missing: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let policy = AutopilotPolicy::default();
        assert!(policy.protected_branches.contains(&"main".to_string()));
        assert!(policy.protected_branches.contains(&"dev".to_string()));
        assert!(policy.fail_on_unrelated_changes);
        assert!(!policy.allow_push);
    }
}
