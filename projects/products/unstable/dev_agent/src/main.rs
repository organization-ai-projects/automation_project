mod diagnostics;
mod patch;
mod plan;
mod public_api;
mod repo;
mod report;
mod verify;

use crate::public_api::{
    AgentError, AgentReport, FileEdit, FileIndex, PatchApplier, Plan, PlanBuilder, RepoRoot,
    RepoScan, Verifier, VerifyStep,
};
use runtime_core::{DeterministicContext, Seed};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let result = match args[1].as_str() {
        "scan" => cmd_scan(&args[2..]),
        "plan" => cmd_plan(&args[2..]),
        "apply" => cmd_apply(&args[2..]),
        "verify" => cmd_verify(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

/// `dev_agent scan [<repo_path>]`
///
/// Scans the repository and emits a `FileIndex` as JSON.
fn cmd_scan(args: &[String]) -> Result<(), AgentError> {
    let path = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let root = RepoRoot::new(path);
    let index = RepoScan::new().scan(&root)?;
    let json = serde_json::to_string_pretty(&index)?;
    println!("{json}");
    Ok(())
}

/// `dev_agent plan [<repo_path>]`
///
/// Produces a deterministic plan JSON by scanning the repo, building the task
/// DAG, scheduling it via `runtime_core::Scheduler`, and recording an event log.
fn cmd_plan(args: &[String]) -> Result<(), AgentError> {
    let path = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let root = RepoRoot::new(path);
    let index = RepoScan::new().scan(&root)?;
    let plan = build_and_schedule_plan(&index)?;
    let json = serde_json::to_string_pretty(&plan)?;
    println!("{json}");
    Ok(())
}

/// `dev_agent apply <repo_path> <patch.json>`
///
/// Applies a patch set (JSON array of `FileEdit`) to the repo and emits an
/// `AgentReport` containing the plan and applied-edits summary.
fn cmd_apply(args: &[String]) -> Result<(), AgentError> {
    let repo_path = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let patch_file = args
        .get(1)
        .ok_or_else(|| AgentError::Io("missing patch file argument".to_string()))?;

    let patch_json = std::fs::read_to_string(patch_file)?;
    let edits: Vec<FileEdit> = serde_json::from_str(&patch_json)?;

    let index = scan_repo(&repo_path)?;
    let plan = build_and_schedule_plan(&index)?;
    let summaries = PatchApplier::new().apply(&repo_path, &edits)?;

    let report = AgentReport::new(plan, summaries, vec![], vec![]);
    let json = serde_json::to_string_pretty(&report)?;
    println!("{json}");
    Ok(())
}

/// `dev_agent verify [<repo_path>] [--fmt] [--clippy] [--test] [--run]`
///
/// Runs verification steps and emits an `AgentReport`.  Without `--run` all
/// steps are recorded as skipped (safe-by-default).
fn cmd_verify(args: &[String]) -> Result<(), AgentError> {
    let repo_path = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let allow_run = args.iter().any(|a| a == "--run");

    let steps = selected_verify_steps(args);
    let index = scan_repo(&repo_path)?;
    let plan = build_and_schedule_plan(&index)?;
    let outcomes = Verifier::new(allow_run).run(&repo_path, &steps)?;

    let report = AgentReport::new(plan, vec![], outcomes, vec![]);
    let json = serde_json::to_string_pretty(&report)?;
    println!("{json}");
    Ok(())
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn scan_repo(path: &Path) -> Result<FileIndex, AgentError> {
    RepoScan::new().scan(&RepoRoot::new(path.to_path_buf()))
}

/// Builds the task DAG from a `FileIndex`, schedules it deterministically via
/// `runtime_core`, and returns the `Plan`.
fn build_and_schedule_plan(index: &FileIndex) -> Result<Plan, AgentError> {
    let plan = PlanBuilder::new().build(index)?;
    let graph = plan.to_graph();
    let mut ctx = DeterministicContext::new(Seed::new(0));
    // Drive the scheduler; the resulting event log is consumed internally.
    ctx.run(graph)?;
    Ok(plan)
}

/// Determines which `VerifyStep`s to run based on CLI flags.
/// Defaults to `fmt` only when no explicit step flag is provided.
fn selected_verify_steps(args: &[String]) -> Vec<VerifyStep> {
    let mut steps = Vec::new();
    if args.iter().any(|a| a == "--fmt") {
        steps.push(VerifyStep::fmt());
    }
    if args.iter().any(|a| a == "--clippy") {
        steps.push(VerifyStep::clippy());
    }
    if args.iter().any(|a| a == "--test") {
        steps.push(VerifyStep::test());
    }
    if steps.is_empty() {
        steps.push(VerifyStep::fmt());
    }
    steps
}

fn print_usage() {
    println!("dev_agent - local repository development agent");
    println!();
    println!("Commands:");
    println!("  scan [<repo_path>]");
    println!("      Scan repo and emit FileIndex as JSON.");
    println!("  plan [<repo_path>]");
    println!("      Produce deterministic plan JSON.");
    println!("  apply <repo_path> <patch.json>");
    println!("      Apply patch set and emit AgentReport JSON.");
    println!("  verify [<repo_path>] [--fmt] [--clippy] [--test] [--run]");
    println!("      Run verification steps and emit AgentReport JSON.");
    println!("      Without --run all steps are skipped (safe default).");
}
