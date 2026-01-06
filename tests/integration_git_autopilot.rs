#[cfg(test)]
mod integration_tests {
    use crate::autopilot::AutopilotPolicy;
    use crate::git_github::handlers::{handle_preview_git_autopilot, PreviewRequest};

    #[test]
    fn test_preview_integration() {
        let request = PreviewRequest {
            policy_overrides: Some(AutopilotPolicy::default()),
        };

        let response = handle_preview_git_autopilot(request);
        assert!(response.is_ok());
        let preview = response.unwrap();
        assert!(preview.report.changes.is_empty()); // Adjust based on expected behavior
    }
}
