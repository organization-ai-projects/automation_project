// projects/products/code_agent_sandbox/src/engine/engine_orchestrator.rs
use crate::{
    actions::ActionResult,
    agents::AgentOutcome,
    command_runner::CommandRunner,
    engine::{
        generate_globs, EngineConfig, EnginePaths, Response, WorkspaceMode, FORBIDDEN, READ, WRITE,
    },
    journal::Journal,
    policy::Policy,
    policy_config::build_policy_config,
    runner_config::RunnerConfig,
    sandbox_fs::SandboxFs,
    score::{ScoreConfig, ScoreSummary},
    worktree,
};
use anyhow::{Context, Result};
use chrono::Utc;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// Structure regroupant les éléments initialisés par l'engine.
pub struct EngineInit {
    pub run_id: String,
    pub run_dir: PathBuf,
    pub work_root: PathBuf,
    pub journal: Journal,
    pub policy: Policy,
    pub sfs: SandboxFs,
    pub runner: CommandRunner,
}

/// Initialise le contexte de l'engine, les répertoires et les politiques.
pub fn initialize_engine(
    paths: &EnginePaths,
    config: &EngineConfig,
    run_id: Option<&str>,
    workspace_mode: WorkspaceMode,
) -> Result<EngineInit> {
    let engine_run_id = match run_id {
        Some(id) if !id.trim().is_empty() => id.trim().to_string(),
        _ => {
            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            format!("{}_{}", timestamp, Uuid::new_v4())
        }
    };

    let run_dir = paths.runs_root.join(&engine_run_id);
    fs::create_dir_all(&run_dir).context("failed to create run_dir")?;

    let work_root = match workspace_mode {
        WorkspaceMode::Learn => {
            let w = run_dir.join("worktree");
            fs::create_dir_all(&w).context("failed to create worktree directory")?;
            w
        }
        WorkspaceMode::Assist => paths.repo_root.clone(),
    };

    let policy = Policy::load_with_overrides(
        build_policy_config(
            paths,
            &run_dir,
            work_root.clone(),
            config,
            generate_globs(FORBIDDEN),
            generate_globs(READ),
            generate_globs(WRITE),
        ),
        &run_dir.join("policy_overrides.toml"),
    )?;

    if matches!(workspace_mode, WorkspaceMode::Learn) {
        worktree::init_worktree_from_repo(&policy)?;
    }

    let journal = Journal::new(run_dir.join("journal.jsonl"))?;
    let sandbox_fs = SandboxFs::new(policy.clone());

    // Configuration sécurisée pour RunnerConfig
    let runner_cfg = RunnerConfig {
        allowed_bins: vec!["cargo".into()],
        allowed_cargo_subcommands: vec![
            "check".into(),
            "test".into(),
            "clippy".into(),
            "fmt".into(),
        ],
        timeout_ms: config.timeout_ms,
        env_allowlist: vec!["RUST_LOG".into()],
    };

    let runner = CommandRunner::new(policy.clone(), runner_cfg);

    Ok(EngineInit {
        run_id: engine_run_id,
        run_dir,
        work_root,
        journal,
        policy,
        sfs: sandbox_fs,
        runner,
    })
}

/// Calcule le score des résultats des actions.
pub fn score_results(results: &[ActionResult]) -> ScoreSummary {
    ScoreSummary::from_results(
        results,
        ScoreConfig {
            penalize_unwrap_outside_tests: true,
            unwrap_penalty: 10,
            penalize_panic_outside_tests: true,
            panic_penalty: 20,
            penalize_dbg_outside_tests: true,
            dbg_penalty: 10,
        },
    )
}

/// Finalise la réponse en consolidant les résultats et le score.
pub fn finalize_response(
    run_id: String,
    workspace_mode: WorkspaceMode,
    work_root: &Path,
    results: Vec<ActionResult>,
    score: ScoreSummary,
    agent_outcome: Option<AgentOutcome>,
) -> Response {
    Response {
        run_id,
        workspace_mode,
        work_root: work_root.to_string_lossy().to_string(),
        results,
        score,
        agent_outcome,
    }
}
