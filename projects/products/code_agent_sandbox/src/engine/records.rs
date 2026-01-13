// projects/products/code_agent_sandbox/src/engine/records.rs
use chrono::Utc;

use crate::{
    actions::{Action, ActionResult},
    journal::Journal,
};

fn record_result(
    journal: &mut Journal,
    run_id: &str,
    result: &ActionResult,
) -> Result<(), anyhow::Error> {
    let t = Utc::now().to_rfc3339();
    journal.record_result(run_id, result, &t)
}

fn push_result(results: &mut Vec<ActionResult>, result: ActionResult) {
    results.push(result);
}

pub fn record_and_push_result(
    journal: &mut Journal,
    run_id: &str,
    result: ActionResult,
    results: &mut Vec<ActionResult>,
) -> Result<(), anyhow::Error> {
    record_result(journal, run_id, &result)?;
    push_result(results, result);
    Ok(())
}

// Specialize record_event for Action and ActionResult
pub fn record_action_event(
    journal: &mut Journal,
    run_id: &str,
    action: &Action,
) -> Result<(), anyhow::Error> {
    let t = Utc::now().to_rfc3339();
    journal.record_action(run_id, action, &t)
}

pub fn check_file_limit(
    files_touched: usize,
    max_files: usize,
    run_id: &str,
    journal: &mut Journal,
    results: &mut Vec<ActionResult>,
) -> Result<bool, anyhow::Error> {
    if files_touched > max_files {
        let res = ActionResult::error(
            "PolicyViolation",
            format!("Too many files touched in one request (>{})", max_files),
        );
        record_and_push_result(journal, run_id, res, results)?;
        return Ok(true);
    }
    Ok(false)
}
