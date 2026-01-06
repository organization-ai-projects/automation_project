/// Plan d’action (ce que l’autopilot ferait).
#[derive(Debug, Clone)]
pub struct AutopilotPlan {
    pub branch: String,
    pub will_stage: Vec<String>,
    pub will_commit: bool,
    pub commit_message: String,
    pub will_push: bool,
    pub notes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autopilot_plan_usage() {
        let plan = AutopilotPlan {
            branch: "main".to_string(),
            will_stage: vec!["file1.rs".to_string()],
            will_commit: true,
            commit_message: "Initial commit".to_string(),
            will_push: true,
            notes: vec!["Note 1".to_string()],
        };

        assert_eq!(plan.branch, "main");
        assert!(plan.will_commit);
        assert_eq!(plan.commit_message, "Initial commit");
        assert!(plan.will_push);
        assert_eq!(plan.notes.len(), 1);
    }
}
