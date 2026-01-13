// projects/products/code_agent_sandbox/src/engine/request.rs
use anyhow::Context;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    actions::{Action, ActionResult},
    agents::{agent_driver, AgentRequest},
    command_runner::CommandRunner,
    engine::{
        generate_globs, run_low_level_actions, EngineConfig, EngineCtx, EnginePaths,
        LowLevelActionContext, Response, WorkspaceMode, FORBIDDEN, READ, WRITE,
    },
    journal::Journal,
    policy::Policy,
    policy_config::build_policy_config,
    runner_config::RunnerConfig,
    sandbox_fs::SandboxFs,
    score::{ScoreConfig, ScoreSummary},
    worktree,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub run_id: Option<String>,

    #[serde(default)]
    pub workspace_mode: WorkspaceMode,

    /// Low-level actions
    #[serde(default)]
    pub actions: Vec<Action>,

    /// High-level request handled by AiOrchestrator
    #[serde(default)]
    pub agent: Option<AgentRequest>,
}

pub fn execute_request(
    req: Request,
    paths: &EnginePaths,
    config: EngineConfig,
) -> Result<Response, anyhow::Error> {
    let run_id = req.run_id.unwrap_or_else(|| Uuid::new_v4().to_string());

    let ts = Utc::now().format("%Y%m%d_%H%M%S_%3f").to_string();
    let run_dir = paths.runs_root.join(format!("{}_{}", ts, &run_id));
    std::fs::create_dir_all(&run_dir).context("failed to create run_dir")?;

    let mut journal = Journal::new(run_dir.join("journal.jsonl"))?;

    let work_root = match req.workspace_mode {
        WorkspaceMode::Assist => paths.repo_root.clone(),
        WorkspaceMode::Learn => {
            let w = run_dir.join("worktree");
            std::fs::create_dir_all(&w).context("failed to create worktree dir")?;
            w
        }
    };

    let forbid_globs = generate_globs(FORBIDDEN);
    let allow_read_globs = generate_globs(READ);
    let allow_write_globs = generate_globs(WRITE);

    let overrides_path = run_dir.join("policy_overrides.toml");
    let policy = Policy::load_with_overrides(
        build_policy_config(
            paths,
            &run_dir,
            work_root.clone(),
            &config,
            forbid_globs,
            allow_read_globs,
            allow_write_globs,
        ),
        &overrides_path,
    )?;

    if matches!(req.workspace_mode, WorkspaceMode::Learn) {
        worktree::init_worktree_from_repo(&policy).context("failed to init worktree")?;
    }

    let sfs = SandboxFs::new(policy.clone());

    let runner = CommandRunner::new(
        policy.clone(),
        RunnerConfig {
            allowed_bins: vec!["cargo".into()],
            allowed_cargo_subcommands: vec![
                "check".into(),
                "test".into(),
                "clippy".into(),
                "fmt".into(),
            ],
            timeout_ms: config.timeout_ms,
            env_allowlist: vec!["RUST_LOG".into()],
        },
    );

    let mut results: Vec<ActionResult> = Vec::new();

    let mut ctx = LowLevelActionContext {
        policy: &policy,
        sfs: &sfs,
        runner: &runner,
        run_dir: &run_dir,
        journal: &mut journal,
        config: &config,
    };

    results.extend(run_low_level_actions(
        &run_id,
        &req.actions,
        &mut ctx, // Utilisation correcte du contexte regroupé
    )?);

    // High-level agent loop (optional)
    let mut agent_outcome = None;
    if let Some(agent_req) = req.agent {
        let mut ctx = EngineCtx {
            run_id: run_id.clone(),
            sfs: sfs.clone(),
            runner: runner.clone(),
            journal: &mut journal,
        };

        let (outcome, agent_results) =
            agent_driver::run_agent_with_orchestrator(&mut ctx, &run_dir, agent_req)?;
        agent_outcome = Some(outcome);
        results.extend(agent_results);
    }

    let score = ScoreSummary::from_results(
        &results,
        ScoreConfig {
            penalize_unwrap_outside_tests: true,
            unwrap_penalty: 10,
            penalize_panic_outside_tests: true,
            panic_penalty: 20,
            penalize_dbg_outside_tests: true,
            dbg_penalty: 10,
        },
    );

    Ok(Response {
        run_id,
        workspace_mode: req.workspace_mode,
        work_root: policy.work_root().to_string_lossy().to_string(),
        results,
        score,
        agent_outcome,
    })
}
