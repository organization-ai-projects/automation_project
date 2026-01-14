// projects/products/code_agent_sandbox/src/agent_driver.rs
use anyhow::{Context, Result};
use common_time::SystemClock;
use protocol::json;
use std::path::Path;

use crate::{
    actions::{Action, ActionResult},
    agents::{AgentOutcome, AgentRequest, STRATEGIES},
    command_runner::extract_cargo_stderr,
    engine::EngineCtx,
    logging::log_event,
    score::{ScoreConfig, ScoreSummary},
};
use ai::AiBody;

/// Parses unified diff (patch) text and returns the list of touched files.
/// This does NOT compute diffs.
pub fn parse_diff_touched_files(unified_diff: &str) -> Vec<String> {
    let mut out = Vec::new();

    for path in unified_diff
        .lines()
        .filter_map(|line| line.strip_prefix("+++ b/").map(str::trim))
        .filter(|p| !p.is_empty() && *p != "/dev/null")
    {
        if !out.iter().any(|x| x == path) {
            out.push(path.to_string());
        }
    }

    out
}


// Updated `run_agent_with_orchestrator` to use refactored functions
pub fn run_agent_with_orchestrator(
    ctx: &mut EngineCtx,
    run_dir: &Path,
    req: AgentRequest,
) -> Result<(AgentOutcome, Vec<ActionResult>)> {
    let mem_path = run_dir.join("agent_memory.jsonl");

    let mut ai_body = AiBody::new().context("AiBody::new failed")?;

    // Adjusted `if let` for compatibility with older Rust versions
    if let Some(replay_path) = &req.replay_path
        && let Err(e) = ai_body.load_training_examples(replay_path)
    {
        tracing::warn!(
            "Failed to load training examples: {:#}. Starting with empty replay buffer.",
            e
        );
    }

    let mut all_results: Vec<ActionResult> = Vec::new();
    let mut last_score = ScoreSummary::from_results(&[], default_score_cfg());
    let mut it = 0usize;

    all_results.extend(run_and_record(
        ctx,
        vec![Action::ListDir {
            path: "src".into(),
            max_depth: 4,
        }],
        run_dir,
    )?);

    if let Some(f) = &req.focus_file {
        all_results.extend(run_and_record(
            ctx,
            vec![Action::ReadFile { path: f.clone() }],
            run_dir,
        )?);
    }

    while it < req.max_iters {
        it += 1;

        let context = build_context_snippet(&all_results, &last_score);
        let prompt = format!(
            "Intent:\n{}\n\nContext:\n{}\n\nTask: propose a minimal Rust code change (unified diff) that makes tests/check pass. Output ONLY a unified diff.",
            req.intent, context
        );

        let task = ai_body.create_task(&prompt);

        let diff_text = if let Some(strategy) = req.forced_strategy.as_deref() {
            let strategy_fn = STRATEGIES
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(strategy.trim()))
                .map(|(_, func)| *func)
                .unwrap_or(AiBody::solve);

            let task_result = strategy_fn(&mut ai_body, &task)?;
            task_result.output
        } else {
            ai_body.solve(&task)?.output
        };

        log_event(
            &mem_path,
            ctx,
            "agent_proposed_diff",
            Some(req.intent.clone()),
            Some(diff_text.clone()),
            None,
        );

        // Added feedback for invalid diffs in `run_agent_with_orchestrator`
        let touched_files = parse_diff_touched_files(&diff_text);
        if touched_files.len() != 1 {
            all_results.push(ActionResult::error(
                "AgentError",
                format!(
                    "Unified diff must touch exactly one file, touched={:?}",
                    touched_files
                ),
            ));

            // Provide negative feedback to the model
            ai_body
                .train_with_verdict(&task, &req.intent, &diff_text, false)
                .ok();

            continue;
        }

        let target_file = touched_files.into_iter().next().unwrap();
        let mut step_results = Vec::new();

        step_results.extend(run_and_record(
            ctx,
            vec![Action::ApplyUnifiedDiff {
                path: target_file.clone(),
                unified_diff: diff_text.clone(),
            }],
            run_dir,
        )?);

        step_results.extend(run_and_record(
            ctx,
            vec![
                Action::RunCargo {
                    subcommand: "check".into(),
                    args: vec![],
                },
                Action::RunCargo {
                    subcommand: "clippy".into(),
                    args: vec!["-q".into()],
                },
                Action::RunCargo {
                    subcommand: "test".into(),
                    args: vec![],
                },
            ],
            run_dir,
        )?);

        all_results.extend(step_results);
        last_score = ScoreSummary::from_results(&all_results, default_score_cfg());

        let ok = last_score.cargo_ok && last_score.score >= 0;
        ai_body
            .train_with_verdict(&task, &req.intent, &diff_text, ok)
            .context("train_with_verdict failed")?;

        log_event(
            &mem_path,
            ctx,
            "agent_feedback",
            Some(req.intent.clone()),
            Some(format!(
                "cargo_ok={} score={} failures={}",
                last_score.cargo_ok, last_score.score, last_score.cargo_failures
            )),
            Some(json::Json::from(json!({
                "cargo_ok": last_score.cargo_ok,
                "score": last_score.score,
                "cargo_failures": last_score.cargo_failures,
                "notes": last_score.notes
            }))),
        );

        if last_score.cargo_ok && last_score.score >= 0 {
            break;
        }
    }

    let outcome = AgentOutcome {
        training_example: None,
        ok: last_score.cargo_ok && last_score.score >= 0,
        iters: it,
        final_score: last_score.score,
        cargo_ok: last_score.cargo_ok,
        cargo_failures: last_score.cargo_failures,
        notes: last_score.notes.clone(),
    };

    Ok((outcome, all_results))
}

// Removed `execute_action` and `run_cargo_action` from this file
use crate::actions::execute_action;

// Updated `run_and_record` to use `common_time::SystemClock`
fn run_and_record(
    ctx: &mut EngineCtx,
    actions: Vec<Action>,
    run_dir: &std::path::Path,
) -> Result<Vec<ActionResult>> {
    let mut results = Vec::with_capacity(actions.len());

    for action in &actions {
        let t = SystemClock::now_rfc3339();
        ctx.journal.record_action(&ctx.run_id, action, &t)?;
        let exec = execute_action(ctx, action, run_dir);

        let res = match exec {
            Ok(ok) => ok,
            Err(e) => ActionResult::error("ExecutionError", format!("{:#}", e)),
        };

        let t = SystemClock::now_rfc3339();
        ctx.journal.record_result(&ctx.run_id, &res, &t)?;
        results.push(res);
    }

    Ok(results)
}

fn default_score_cfg() -> ScoreConfig {
    ScoreConfig {
        penalize_unwrap_outside_tests: true,
        unwrap_penalty: 10,
        penalize_panic_outside_tests: true,
        panic_penalty: 20,
        penalize_dbg_outside_tests: true,
        dbg_penalty: 10,
    }
}

// Simplified build_context_snippet
fn build_context_snippet(results: &[ActionResult], score: &ScoreSummary) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "score={}, cargo_ok={}, failures={}",
        score.score, score.cargo_ok, score.cargo_failures
    ));

    parts.extend(score.notes.iter().take(10).map(|n| format!("note: {n}")));

    parts.extend(
        results
            .iter()
            .rev()
            .take(20)
            .filter_map(extract_cargo_stderr),
    );

    parts.join("\n")
}
