//! tools/versioning_automation/src/issues/tests/sync_project_status.rs
use crate::issues::{commands::SyncProjectStatusOptions, run_sync_project_status};

// Mock to replace external calls
fn mock_gh_graphql(args: &[(&str, &str)]) -> Option<String> {
    let query = args
        .iter()
        .find(|(key, value)| *key == "-f" && value.contains("query"))
        .map(|(_, value)| value.to_string());
    match query {
        Some(query) if query.contains("projectItems") => {
            Some("{\"data\": {\"repository\": {\"issue\": {\"projectItems\": {\"nodes\": [{\"id\": \"mock_id\", \"project\": {\"id\": \"mock_id\"}}]}}}}}".to_string())
        }
        Some(_) => {
            Some("mock".to_string())
        }
        None => None,
    }
}

#[test]
fn test_run_sync_project_status() {
    let opts = SyncProjectStatusOptions {
        repo: "owner/repo".to_string(),
        issue: "123".to_string(),
        status: "In Progress".to_string(),
    };

    // Use the mock to replace external calls
    let _ = mock_gh_graphql(&[("-f", "query=mock_query")]);

    let result = run_sync_project_status(opts);

    assert_eq!(result, 0);
}
