//! projects/products/stable/code_agent_sandbox/backend/src/tests/journal.rs
use std::fs;

use crate::actions::{Action, ActionResult};
use crate::journal::Journal;

#[test]
fn journal_writes_action_and_result_lines() {
    let file = tempfile::NamedTempFile::new().expect("temp file");
    let path = file.path().to_path_buf();

    let mut journal = Journal::new(path.clone()).expect("journal should open");
    let action = Action::ReadFile {
        path: "src/main.rs".to_string(),
    };
    let result = ActionResult::success("ReadFile", "ok", None);

    journal
        .record_action("run-1", &action, "2026-01-01T00:00:00Z")
        .expect("record action");
    journal
        .record_result("run-1", &result, "2026-01-01T00:00:01Z")
        .expect("record result");

    drop(journal);

    let content = fs::read_to_string(path).expect("read journal");
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("\"event\":\"action\""));
    assert!(lines[1].contains("\"event\":\"result\""));
}
