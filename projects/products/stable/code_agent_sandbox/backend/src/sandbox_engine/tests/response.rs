//! projects/products/stable/code_agent_sandbox/backend/src/sandbox_engine/tests/response.rs
use crate::actions::ActionResult;
use crate::sandbox_engine::{Response, WorkspaceMode};
use crate::score::ScoreSummary;

#[test]
fn response_new_preserves_payload_fields() {
    let response = Response::new(
        "run-1".to_string(),
        WorkspaceMode::Assist,
        "/tmp/work".to_string(),
        vec![ActionResult::success("ReadFile", "ok", None)],
        ScoreSummary::default(),
        None,
    );

    assert_eq!(response.run_id, "run-1");
    assert_eq!(response.workspace_mode, WorkspaceMode::Assist);
    assert_eq!(response.work_root, "/tmp/work");
    assert_eq!(response.results.len(), 1);
    assert!(response.agent_outcome.is_none());
}
