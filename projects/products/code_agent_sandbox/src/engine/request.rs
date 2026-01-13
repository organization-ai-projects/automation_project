use std::fs;

// projects/products/code_agent_sandbox/src/engine/request.rs
use anyhow::Context;
use serde::Deserialize;

use crate::{
    actions::{Action, ActionResult, LowLevelActionContext, run_low_level_actions},
    agents::{AgentRequest, agent_driver},
    engine::{
        EngineConfig, EngineCtx, EngineInit, EnginePaths, Response, WorkspaceMode
    },
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

/// ✅ Core du domaine.
/// ⚠️ Important: pas callable depuis “dehors” du module engine.
pub(in crate::engine) fn execute_with_init(
    mut req: Request,
    init: &mut EngineInit,
    paths: &EnginePaths,
    config: &EngineConfig,
) -> Result<Response, anyhow::Error> {
    // Orchestrator a décidé du run_id (source of truth)
    req.run_id = Some(init.run_id.clone());

    let mut results: Vec<ActionResult> = Vec::new();

    // Low-level actions
    let mut ll_ctx = LowLevelActionContext {
        policy: &init.policy,
        sfs: &init.sfs,
        runner: &init.runner,
        run_dir: &init.run_dir,
        journal: &mut init.journal,
        config,
    };

    results.extend(run_low_level_actions(
        &init.run_id,
        &req.actions,
        &mut ll_ctx,
    )?);

    // High-level agent loop (optional)
    let mut agent_outcome = None;
    if let Some(mut agent_req) = req.agent.take() {
        // ✅ on garde ton comportement “modèle + replay global”
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
            agent_driver::run_agent_with_orchestrator(&mut ctx, &init.run_dir, agent_req)?;
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

    Ok(Response {
        run_id: init.run_id.clone(),
        workspace_mode: req.workspace_mode,
        work_root: init.work_root.to_string_lossy().to_string(),
        results,
        score,
        agent_outcome,
    })
}
