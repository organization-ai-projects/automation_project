// projects/products/code_agent_sandbox/src/engine/engine_init.rs
use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;

use common::Id128;

use crate::{
    command_runner::CommandRunner,
    engine::{EngineConfig, EnginePaths, FORBIDDEN, READ, WRITE, WorkspaceMode, generate_globs},
    execution_paths::ExecutionPaths,
    journal::Journal,
    policies::{Policy, PolicyConfig},
    runner_config::RunnerConfig,
    sandbox_fs::SandboxFs,
    worktree,
};

/// ✅ Type de domaine, pas une façade.
/// L’orchestrateur construit ceci et le passe au cœur.
pub struct EngineInit {
    pub run_id: String,
    pub paths: ExecutionPaths,
    pub journal: Journal,
    pub policy: Policy,
    pub sfs: SandboxFs,
    pub runner: CommandRunner,
}

pub fn initialize_engine(
    paths: &EnginePaths,
    config: &EngineConfig,
    run_id: Option<&str>,
    workspace_mode: WorkspaceMode,
) -> Result<EngineInit> {
    let engine_run_id = match run_id {
        Some(id) if !id.trim().is_empty() => id.trim().to_string(),
        _ => {
            let ts = Utc::now().format("%Y%m%d_%H%M%S_%3f").to_string();
            format!("{}_{}", ts, Id128::new(0, None, None))
        }
    };

    let run_dir = paths.runs_root.join(&engine_run_id);
    fs::create_dir_all(&run_dir).context("failed to create run_dir")?;

    let work_root = match workspace_mode {
        WorkspaceMode::Assist => paths.repo_root.clone(),
        WorkspaceMode::Learn => {
            let w = run_dir.join("worktree");
            fs::create_dir_all(&w).context("failed to create worktree dir")?;
            w
        }
    };

    let overrides_path = run_dir.join("policy_overrides.toml");

    let policy = Policy::load_with_overrides(
        PolicyConfig::new(
            paths,
            &run_dir,
            work_root.clone(),
            config,
            generate_globs(FORBIDDEN),
            generate_globs(READ),
            generate_globs(WRITE),
        ),
        &overrides_path,
    )?;

    if matches!(workspace_mode, WorkspaceMode::Learn) {
        worktree::init_worktree_from_repo(&policy).context("failed to init worktree")?;

        // ton check de repo git (tu l’avais)
        if !work_root.join(".git").exists() {
            anyhow::bail!("work_root is not a valid Git repository");
        }
    }

    let journal = Journal::new(run_dir.join("journal.jsonl"))?;
    let sfs = SandboxFs::new(policy.clone());

    let runner_cfg = RunnerConfig {
        allowed_cargo_subcommands: vec![
            "check".into(),
            "test".into(),
            "clippy".into(),
            "fmt".into(),
        ],
        cargo_path: "cargo".into(),
    };

    let runner = CommandRunner::new(policy.clone(), runner_cfg);

    Ok(EngineInit {
        run_id: engine_run_id,
        paths: ExecutionPaths::new(run_dir, work_root),
        journal,
        policy,
        sfs,
        runner,
    })
}
