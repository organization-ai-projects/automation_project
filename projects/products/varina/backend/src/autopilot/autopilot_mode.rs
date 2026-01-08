use serde::{Deserialize, Serialize};

/// Mode d'exécution de l'autopilot.
/// - `DryRun` : Génère un plan sans appliquer de changements.
/// - `ApplySafe` : Applique les changements uniquement si les vérifications passent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutopilotMode {
    DryRun,
    ApplySafe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autopilot_mode_usage() {
        let mode = AutopilotMode::DryRun;
        assert_eq!(mode, AutopilotMode::DryRun);

        let mode = AutopilotMode::ApplySafe;
        assert_eq!(mode, AutopilotMode::ApplySafe);
    }
}
