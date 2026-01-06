/// Mode d'autopilot.
/// - DryRun: ne modifie rien, renvoie un plan.
/// - ApplySafe: applique uniquement si c'est sûr (policy + checks OK).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
