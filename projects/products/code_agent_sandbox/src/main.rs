// projects/products/code_agent_sandbox/src/main.rs
mod actions;
mod agents;
mod command_runner;
mod engine;
mod journal;
mod memory;
mod normalization;
mod policy;
mod policy_config;
mod runner_config;
mod sandbox_fs;
mod score;
mod worktree;

use crate::agents::agent_driver;
use crate::engine::engine_orchestrator;
use crate::engine::{EngineConfig, EnginePaths, LowLevelActionContext, Request};
use ai::ai_body::AiBody;
use ai::task::Task;
use anyhow::{bail, Context, Result};
use std::io::{self, Read};
use std::path::PathBuf;
use tracing::warn;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <repo_root> <runs_root>",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("code_agent_sandbox")
        );
        bail!("invalid arguments");
    }

    let paths = EnginePaths {
        repo_root: PathBuf::from(&args[1]),
        runs_root: PathBuf::from(&args[2]),
    };

    let config = EngineConfig::default();

    // stdin -> Request
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let req: Request = serde_json::from_str(&input).context("Invalid JSON input for Request")?;

    // IA obligatoire
    if req.agent.is_none() {
        bail!("agent is required: AI is not optional");
    }

    // 1) Engine init (ENGINE ONLY)
    let workspace_mode = req.workspace_mode.clone();

    let mut init = engine_orchestrator::initialize_engine(
        &paths,
        &config,
        req.run_id.as_deref(), // Respecte le run_id fourni dans la requête
        workspace_mode,
    )?;

    // 2) Low-level actions (ENGINE ONLY)
    let mut results = Vec::new();
    {
        let mut ll_ctx = LowLevelActionContext {
            policy: &init.policy,
            sfs: &init.sfs,
            runner: &init.runner,
            run_dir: &init.run_dir,
            journal: &mut init.journal,
            config: &config,
        };

        let low_level = engine::run_low_level_actions(&init.run_id, &req.actions, &mut ll_ctx)?;
        results.extend(low_level);
    }

    // Ensure models/ and replay.jsonl paths exist
    let model_dir = paths.runs_root.join("models");
    let replay_path = paths.runs_root.join("replay.jsonl");

    std::fs::create_dir_all(&model_dir).context("Failed to create models directory")?;
    if !replay_path.exists() {
        std::fs::File::create(&replay_path).context("Failed to create replay file")?;
    }

    // Pass paths to the agent request
    let mut agent_req = req.agent.clone().unwrap(); // safe: checked above
    agent_req.model_dir = Some(model_dir.clone());
    agent_req.replay_path = Some(replay_path.clone());

    // 3) Agent IA (APP orchestration, donc main.rs)
    let (agent_outcome, agent_results) = {
        let mut agent_ctx = crate::engine::EngineCtx {
            run_id: init.run_id.clone(),
            sfs: init.sfs.clone(),
            runner: init.runner.clone(),
            journal: &mut init.journal,
        };

        agent_driver::run_agent_with_orchestrator(&mut agent_ctx, &init.run_dir, agent_req)?
    };

    results.extend(agent_results);

    // 4) Score + Response (ENGINE ONLY helpers)
    let score = engine_orchestrator::score_results(&results);

    // Cloner agent_outcome avant de le déplacer
    let agent_outcome_clone = agent_outcome.clone();

    // Passer à l'agent outcome cloné ici
    let resp = engine_orchestrator::finalize_response(
        init.run_id,
        req.workspace_mode,
        &init.work_root,
        results,
        score,
        Some(agent_outcome),
    );

    println!("{}", serde_json::to_string_pretty(&resp)?);

    // Vérification et centralisation des interactions via AiBody
    // Suppression des interactions directes inutiles avec d'autres modules
    // Charger les exemples d'entraînement existants
    let mut ai_body = AiBody::new()?;
    let training_examples = ai_body
        .load_training_examples(&replay_path)
        .unwrap_or_else(|_| {
            warn!("Failed to load training examples. Starting with an empty replay buffer.");
            vec![]
        });

    // Utilisation de training_examples pour éviter l'erreur
    tracing::info!("Loaded {} training examples", training_examples.len());

    // Ajouter un exemple d'entraînement après une action réussie
    if let Some(example) = agent_outcome_clone.training_example.as_ref() {
        // Utilisation correcte de example
        let task = Task::new_code_generation("example input".to_string());
        ai_body
            .train_with_verdict(
                &task,
                "example input",
                example,
                true, // Exemple correct
            )
            .unwrap_or_else(|e| {
                warn!("Failed to train with verdict: {:?}", e);
            });
    }

    // Sauvegarder le modèle neural après l'entraînement
    let model_path = model_dir.join("neural_model.bin");
    let tokenizer_path = model_dir.join("tokenizer.bin");
    ai_body
        .save_neural_model(&model_path, &tokenizer_path)
        .unwrap_or_else(|e| {
            warn!("Failed to save neural model: {:?}", e);
        });

    // Exemple d'utilisation de AiBody pour résoudre une tâche
    let task = Task::new_code_generation("example task".to_string());
    let result = ai_body.solve(&task)?;
    tracing::info!("Résultat de la tâche: {:?}", result);

    Ok(())
}
