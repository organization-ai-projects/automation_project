//! projects/products/stable/code_agent_sandbox/backend/src/score/tests/score_summary.rs

use common_json::pjson;

use crate::actions::ActionResult;
use crate::score::{ScoreConfig, ScoreSummary};

#[test]
fn score_summary_penalizes_unwrap_outside_tests() {
    let results = vec![ActionResult::success(
        "ReadFile",
        "ok",
        Some(pjson!({
            "path": "src/lib.rs",
            "contents": "fn f() { let _ = x.unwrap(); }"
        })),
    )];

    let cfg = ScoreConfig {
        penalize_unwrap_outside_tests: true,
        unwrap_penalty: 10,
        penalize_panic_outside_tests: false,
        panic_penalty: 0,
        penalize_dbg_outside_tests: false,
        dbg_penalty: 0,
    };

    let summary = ScoreSummary::from_results(&results, cfg);
    assert_eq!(summary.score, -10);
    assert!(!summary.notes.is_empty());
}
