use ai::AiBody;
// projects/products/code_agent_sandbox/src/agent_driver.rs
use anyhow::{Context, Result};
use chrono::Utc;

use crate::{
    actions::{Action, ActionResult},
    agents::{AgentOutcome, AgentRequest},
    command_runner::CommandRunner,
    engine::EngineCtx,
    memory::{append_event, MemoryEvent},
    score::{ScoreConfig, ScoreSummary},
};

pub fn run_agent_with_orchestrator(
    ctx: &mut EngineCtx,
    run_dir: &std::path::Path,
    req: AgentRequest,
) -> Result<(AgentOutcome, Vec<ActionResult>)> {
    let mem_path = run_dir.join("agent_memory.jsonl");

    let mut ai_body = AiBody::new().context("AiBody::new failed")?;

    // Charger les exemples d'entraînement existants
    if let Some(replay_path) = &req.replay_path {
        let _ = ai_body
            .load_training_examples(replay_path)
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to load training examples. Starting with an empty replay buffer."
                );
                vec![]
            });
    }

    // Charger le modèle neural si disponible
    if let Some(model_dir) = &req.model_dir {
        let model_path = model_dir.join("neural_model.bin");
        let tokenizer_path = model_dir.join("tokenizer.json");
        if model_path.exists() && tokenizer_path.exists() {
            let _ = ai_body.load_neural_model(&model_path, &tokenizer_path);
        }
    }

    let mut all_results: Vec<ActionResult> = Vec::new();
    let mut last_score = ScoreSummary::from_results(&[], default_score_cfg());
    let mut it = 0usize;

    // Warmup: list src + maybe read focus_file
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

        // 1) Ask orchestrator for code output based on intent + last feedback
        // We pass a compact context string built from recent cargo outputs + notes.
        let context = build_context_snippet(&all_results, &last_score);

        let prompt = format!(
            "Intent:\n{}\n\nContext:\n{}\n\nTask: propose a minimal Rust code change (unified diff) that makes tests/check pass. Output ONLY a unified diff.",
            req.intent,
            context
        );

        let task = ai_body.create_task(&prompt);

        let diff_text = match req.forced_strategy.as_deref() {
            None => ai_body.solve(&task)?.output,

            Some(s) => {
                let s = s.trim().to_lowercase();
                match s.as_str() {
                    "auto" => ai_body.solve(&task)?.output,
                    "symbolicthenneural" | "symbolic_then_neural" | "sym_then_neu" => {
                        ai_body.solve_symbolic_then_neural(&task)?.output
                    }
                    "neuralwithsymbolicvalidation" | "neural_with_validation" | "neu_validate" => {
                        ai_body.solve_neural_with_validation(&task)?.output
                    }
                    "hybrid" => ai_body.solve_hybrid(&task)?.output,
                    _ => ai_body.solve(&task)?.output,
                }
            }
        };

        append_event(
            &mem_path,
            &MemoryEvent {
                ts: Utc::now().to_rfc3339(),
                run_id: ctx.run_id.clone(),
                kind: "agent_proposed_diff".into(),
                input: Some(req.intent.clone()),
                output: Some(diff_text.clone()),
                metadata: None,
            },
        )
        .ok();

        // 2) Apply diff (we need a target path; simplest: if diff contains "+++ b/..."
        // but your Action requires explicit path. So we do a pragmatic approach:
        // - if focus_file provided => apply diff to that path
        // - else => ask IA to output diff for ONE file and we detect it
        let target_file = req
            .focus_file
            .clone()
            .or_else(|| detect_single_target_file(&diff_text));

        let mut step_results = Vec::new();

        if let Some(path) = target_file {
            step_results.extend(run_and_record(
                ctx,
                vec![Action::ApplyUnifiedDiff {
                    path: path.clone(),
                    unified_diff: diff_text.clone(),
                }],
                run_dir, // Ajout du paramètre run_dir
            )?);
        } else {
            step_results.push(ActionResult::error(
                "AgentError",
                "Could not detect target file for unified diff. Provide focusFile or output a diff touching a single file with clear headers.",
            ));
        }

        // 3) Run cargo check + clippy + test
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

        // 4) Score
        last_score = ScoreSummary::from_results(&all_results, default_score_cfg());

        // 5) Convert score into learning signal
        // If cargo_ok and score positive => Correct, else Incorrect
        let ok = last_score.cargo_ok && last_score.score >= 0;
        ai_body
            .train_with_verdict(&task, &req.intent, &diff_text, ok)
            .context("train_with_verdict failed")?;

        append_event(
            &mem_path,
            &MemoryEvent {
                ts: Utc::now().to_rfc3339(),
                run_id: ctx.run_id.clone(),
                kind: "agent_feedback".into(),
                input: Some(req.intent.clone()),
                output: Some(format!(
                    "cargo_ok={} score={} failures={}",
                    last_score.cargo_ok, last_score.score, last_score.cargo_failures
                )),
                metadata: Some(protocol::json!({
                    "cargo_ok": last_score.cargo_ok,
                    "score": last_score.score,
                    "cargo_failures": last_score.cargo_failures,
                    "notes": last_score.notes
                })),
            },
        )
        .ok();

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

pub fn run_and_record(
    ctx: &mut EngineCtx,
    actions: Vec<Action>,
    run_dir: &std::path::Path,
) -> Result<Vec<ActionResult>> {
    let mut results = Vec::with_capacity(actions.len());

    for action in &actions {
        let t = Utc::now().to_rfc3339();
        ctx.journal.record_action(&ctx.run_id, action, &t)?;
        let exec = match action {
            Action::ReadFile { path } => ctx.sfs.read_file(path),
            Action::ListDir { path, max_depth } => ctx.sfs.list_dir(path, *max_depth),
            Action::WriteFile {
                path,
                contents,
                create_dirs,
            } => ctx.sfs.write_file(path, contents, *create_dirs),
            Action::ApplyUnifiedDiff { path, unified_diff } => {
                ctx.sfs.apply_unified_diff(path, unified_diff)
            }
            Action::RunCargo { subcommand, args } => {
                if CommandRunner::requires_bunker(subcommand) {
                    ctx.runner
                        .run_in_bunker("cargo", &[subcommand.clone(), args.join(" ")])
                } else {
                    ctx.runner.run_cargo(subcommand, args)
                }
            }
            Action::GenerateCode { language, code } => {
                // We keep it harmless: save only.
                let ai_ws = run_dir.join("ai_workspace");
                std::fs::create_dir_all(&ai_ws)?;
                let file_path =
                    ai_ws.join(format!("generated_{}.{}", uuid::Uuid::new_v4(), language));
                std::fs::write(&file_path, code)?;
                Ok(ActionResult::success(
                    "CodeGenerated",
                    "saved",
                    Some(protocol::json!({ "path": file_path.to_string_lossy() })),
                ))
            }
        };

        let res = match exec {
            Ok(ok) => ok,
            Err(e) => ActionResult::error("ExecutionError", format!("{:#}", e)),
        };

        let t = Utc::now().to_rfc3339();
        ctx.journal.record_result(&ctx.run_id, &res, &t)?;
        results.push(res);
    }

    Ok(results)
}

pub fn default_score_cfg() -> ScoreConfig {
    ScoreConfig {
        penalize_unwrap_outside_tests: true,
        unwrap_penalty: 10,
        penalize_panic_outside_tests: true,
        panic_penalty: 20,
        penalize_dbg_outside_tests: true,
        dbg_penalty: 10,
    }
}

pub fn build_context_snippet(results: &[ActionResult], score: &ScoreSummary) -> String {
    // Keep it short: last cargo stderr snippets + notes
    let mut parts = Vec::new();
    parts.push(format!(
        "score={}, cargo_ok={}, failures={}",
        score.score, score.cargo_ok, score.cargo_failures
    ));

    for n in score.notes.iter().take(10) {
        parts.push(format!("note: {n}"));
    }

    // Extract last cargo stderr
    for r in results.iter().rev().take(20) {
        if r.kind.starts_with("Cargo") {
            if let Some(data) = &r.data {
                if let Some(stderr) = data.get("stderr").and_then(|v| v.as_str()) {
                    let s = truncate(stderr, 2000);
                    parts.push(format!("{} stderr:\n{}", r.kind, s));
                }
            }
        }
    }

    parts.join("\n")
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut t = s[..max].to_string();
    t.push_str("\n...[truncated]...");
    t
}

pub fn detect_single_target_file(unified_diff: &str) -> Option<String> {
    // Very simple: look for a line like "+++ b/src/main.rs"
    // and extract path after "b/".
    for line in unified_diff.lines() {
        if let Some(rest) = line.strip_prefix("+++ b/") {
            let p = rest.trim();
            if !p.is_empty() {
                return Some(p.to_string());
            }
        }
    }
    None
}
