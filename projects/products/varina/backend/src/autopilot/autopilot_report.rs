use crate::{
    autopilot::{AutopilotMode, AutopilotPlan},
    classified_changes::ClassifiedChanges,
    git_github::GitChange,
};

/// Rapport d’exécution (plan + actions réalisées ou refus).
#[derive(Debug, Clone)]
pub struct AutopilotReport {
    pub mode: AutopilotMode,
    pub branch: String,
    pub detached_head: bool,
    pub changes: Vec<GitChange>,
    pub classified: ClassifiedChanges,
    pub plan: AutopilotPlan,
    pub applied: bool,
    pub logs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autopilot_report_usage() {
        let report = AutopilotReport {
            mode: AutopilotMode::DryRun,
            branch: "main".to_string(),
            detached_head: false,
            changes: vec![],
            classified: ClassifiedChanges {
                blocked: vec![GitChange {
                    xy: [b'M', b' '],
                    path: "blocked_file.rs".to_string(),
                    orig_path: None,
                }],
                relevant: vec![GitChange {
                    xy: [b'A', b' '],
                    path: "relevant_file.rs".to_string(),
                    orig_path: None,
                }],
                unrelated: vec![GitChange {
                    xy: [b'D', b' '],
                    path: "unrelated_file.rs".to_string(),
                    orig_path: None,
                }],
            },
            plan: AutopilotPlan {
                branch: "main".to_string(),
                will_stage: vec!["file1.rs".to_string()],
                will_commit: true,
                commit_message: "Initial commit".to_string(),
                will_push: true,
                notes: vec!["Note 1".to_string()],
            },
            applied: true,
            logs: vec!["Log entry".to_string()],
        };

        assert_eq!(report.mode, AutopilotMode::DryRun);
        assert_eq!(report.branch, "main");
        assert!(!report.detached_head);
        assert!(report.changes.is_empty());
        assert!(report.applied);
        assert_eq!(report.logs.len(), 1);

        // Utilisation des champs classified et plan
        assert_eq!(report.classified.blocked[0].path, "blocked_file.rs");
        assert_eq!(report.classified.relevant[0].path, "relevant_file.rs");
        assert_eq!(report.classified.unrelated[0].path, "unrelated_file.rs");

        assert_eq!(report.plan.branch, "main");
        assert!(report.plan.will_commit);
        assert_eq!(report.plan.commit_message, "Initial commit");
        assert!(report.plan.will_push);
        assert_eq!(report.plan.notes.len(), 1);
    }
}
