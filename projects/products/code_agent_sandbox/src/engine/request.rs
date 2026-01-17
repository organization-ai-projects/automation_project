// projects/products/code_agent_sandbox/src/engine/request.rs
use anyhow::{Context, anyhow};
use common_time::SystemClock;
use common_time::timeout::with_timeout;
use serde::Deserialize;
use std::fs;

use crate::{
    actions::{Action, ActionResult, LowLevelActionContext, run_low_level_actions},
    agents::{AgentRequest, run_agent_with_orchestrator},
    engine::{EngineConfig, EngineCtx, EngineInit, EnginePaths, Response, WorkspaceMode},
    score::{ScoreConfig, ScoreSummary},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub run_id: Option<String>,

    #[serde(default)]
    pub workspace_mode: WorkspaceMode,

    #[serde(default)]
    pub actions: Vec<Action>,

    #[serde(default)]
    pub agent: Option<AgentRequest>,
}

/// ✅ Core of the domain.
/// ⚠️ Important: not callable from "outside" the engine module.
pub(in crate::engine) async fn execute_with_init(
    mut req: Request,
    init: &mut EngineInit,
    paths: &EnginePaths,
    config: &EngineConfig,
) -> Result<Response, anyhow::Error> {
    // Orchestrator decided the run_id (source of truth)
    req.run_id = Some(init.run_id.clone());

    let mut results: Vec<ActionResult> = Vec::new();

    // Low-level actions
    let mut ll_ctx = LowLevelActionContext {
        policy: &init.policy,
        sfs: &init.sfs,
        runner: &init.runner,
        run_dir: &init.paths.run_dir,
        journal: &mut init.journal,
        config,
    };

    let timeout = config.timeout; // Direct use of the timeout field, which is now mandatory
    let clock = SystemClock;
    let result = with_timeout(
        async { run_low_level_actions(&init.run_id, &req.actions, &mut ll_ctx) },
        &clock,
        timeout,
    )
    .await;

    match result {
        Ok(actions_result) => {
            results.extend(actions_result?);
        }
        Err(_) => {
            return Err(anyhow!(
                "Execution timed out after {:?}",
                timeout.as_duration()
            ));
        }
    }

    // High-level agent loop (optional)
    let mut agent_outcome = None;
    if let Some(mut agent_req) = req.agent.take() {
        // ✅ keeping your behavior "global model + replay"
        let model_dir = paths.runs_root.join("models");
        let replay_path = paths.runs_root.join("replay.jsonl");

        fs::create_dir_all(&model_dir).context("failed to create models dir")?;
        if !replay_path.exists() {
            fs::File::create(&replay_path).context("failed to create replay file")?;
        }

        agent_req.model_dir = Some(model_dir);
        agent_req.replay_path = Some(replay_path);

        let mut ctx = EngineCtx {
            run_id: init.run_id.clone(),
            sfs: init.sfs.clone(),
            runner: init.runner.clone(),
            journal: &mut init.journal,
        };

        let (outcome, agent_results) =
            run_agent_with_orchestrator(&mut ctx, &init.paths.run_dir, agent_req)?;
        agent_outcome = Some(outcome);
        results.extend(agent_results);
    }

    // Score
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

    Ok(Response::new(
        init.run_id.clone(),
        req.workspace_mode,
        init.paths.work_root.to_string_lossy().to_string(),
        results,
        score,
        agent_outcome,
    ))
}
