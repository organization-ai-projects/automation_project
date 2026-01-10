// projects/products/code_agent_sandbox/src/main.rs
use std::{
    io::{self, Read},
    path::PathBuf,
};

use ai::{ai_orchestrator::AiOrchestrator, solver_strategy::SolverStrategy, task::Task};
use anyhow::{Context, Result};
use chrono::Utc;
use code_agent_sandbox::{
    policy::PolicyConfig, runner::RunnerConfig, Action, ActionResult, CommandRunner, Journal,
    Policy, SandboxFs, ScoreConfig, ScoreSummary,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// This program is a "tampon" (gateway) between an AI agent and your machine.
/// It executes a constrained set of actions (file reads/writes/patch + cargo commands)
/// inside an allowlisted root, logs everything, and returns structured results.
///
/// Usage:
///   echo '<json request>' | cargo run --release -- <repo_root> <run_root>
/// Example:
///   echo '{"run_id":null,"actions":[{"kind":"RunCargo","args":{"subcommand":"check","args":[]}}]}' \
///     | cargo run --release -- ./my_repo ./.agent_runs
fn main() -> Result<()> {
    // Définir les chemins par défaut pour repo_root et runs_root avec des chemins relatifs
    let repo_root = PathBuf::from("./");
    let runs_root = PathBuf::from("./projects/products/code_agent_sandbox");

    // Lire l'entrée JSON complète depuis stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut req: Request = serde_json::from_str(&input).context("invalid JSON request")?;

    // Create run id if absent
    let run_id = req
        .run_id
        .take()
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Build run directory
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let run_dir = runs_root.join(format!("{}_{}", timestamp, run_id));
    std::fs::create_dir_all(&run_dir)?;

    // Set up policy
    let policy = Policy::new(PolicyConfig {
        repo_root: repo_root.clone(),
        run_dir: run_dir.clone(),
        // Tight defaults (edit these to your taste)
        max_read_bytes: 1_500_000,
        max_write_bytes: 2_000_000,
        max_files_per_request: 80,
        forbid_globs: vec![
            ".git/**".into(),
            "**/.env".into(),
            "**/.env.*".into(),
            "**/id_rsa".into(),
            "**/id_ed25519".into(),
            "**/secrets/**".into(),
            "**/target/**".into(),
        ],
        allow_write_globs: vec![
            "src/**".into(),
            "tests/**".into(),
            "examples/**".into(),
            "benches/**".into(),
            "Cargo.toml".into(),
            "README.md".into(),
        ],
        allow_read_globs: vec![
            "**".into(), // read is broad, still filtered by forbid_globs + size limits
        ],
    })?;

    // File system gateway
    let sfs = SandboxFs::new(policy.clone());

    // Ajouter un espace dédié pour l'IA
    let ai_workspace = runs_root.join("ai_workspace");
    std::fs::create_dir_all(&ai_workspace)?;

    // Étendre le CommandRunner pour autoriser l'exécution de scripts dans l'espace IA
    let runner = CommandRunner::new(
        policy.clone(),
        RunnerConfig {
            allowed_bins: vec!["cargo".into(), "python3".into()], // Autoriser Python
            allowed_cargo_subcommands: vec![
                "check".into(),
                "test".into(),
                "clippy".into(),
                "fmt".into(),
            ],
            timeout_ms: 120_000,
            env_allowlist: vec!["RUST_LOG".into()],
        },
    );

    // Journal
    let mut journal = Journal::new(run_dir.join("journal.jsonl"))?;

    // Initialiser l'orchestrateur AI
    let mut ai_orchestrator = AiOrchestrator::new()?;

    // Charger un modèle neuronal si nécessaire
    let model_path = PathBuf::from("./models/neural_model.bin");
    let tokenizer_path = PathBuf::from("./models/tokenizer.json");
    if model_path.exists() && tokenizer_path.exists() {
        ai_orchestrator.load_neural_model(&model_path, &tokenizer_path)?;
    }

    // Exemple d'utilisation de l'orchestrateur pour résoudre une tâche
    let task = Task::new_code_generation("fn main() { println!(\"Hello, world!\"); }".into());
    let result = ai_orchestrator.solve(&task)?;
    println!("Résultat de la tâche : {:?}", result);

    // Exemple d'utilisation de l'approche hybride
    let hybrid_task =
        Task::new_code_generation("fn main() { println!(\"Hello, Hybrid AI!\"); }".into());
    let hybrid_result = ai_orchestrator.solve_forced(&hybrid_task, SolverStrategy::Hybrid)?;
    println!("Résultat hybride : {:?}", hybrid_result);

    // Execute actions
    let mut results = Vec::with_capacity(req.actions.len());
    let mut files_touched = 0usize;

    // Utiliser une référence pour éviter le déplacement
    for action in &req.actions {
        files_touched += action.estimated_file_touch_count();
        if files_touched > policy.config().max_files_per_request {
            results.push(ActionResult::error(
                "PolicyViolation",
                format!(
                    "Too many files touched in one request (>{})",
                    policy.config().max_files_per_request
                ),
            ));
            break;
        }

        let timestamp = Utc::now().to_rfc3339();
        journal.record_action(&run_id, action, &timestamp)?;

        let res = match action {
            Action::ReadFile { path } => sfs.read_file(path),
            Action::ListDir { path, max_depth } => sfs.list_dir(path, *max_depth),
            Action::WriteFile {
                path,
                contents,
                create_dirs,
            } => sfs.write_file(path, contents, *create_dirs),
            Action::ApplyUnifiedDiff { path, unified_diff } => {
                sfs.apply_unified_diff(path, unified_diff)
            }
            Action::RunCargo { subcommand, args } => runner.run_cargo(subcommand, args),
            Action::GenerateCode { language, code } => {
                let language = language.to_lowercase();
                let file_path = ai_workspace.join(format!("generated_{}.{}", timestamp, language));
                std::fs::write(&file_path, code)?;

                if language == "python" {
                    runner.run_command("python3", &[file_path.to_string_lossy().to_string()])
                } else if language == "rust" {
                    let output_binary =
                        ai_workspace.join(format!("generated_{}_binary", timestamp));
                    let compile_status = runner.run_command(
                        "rustc",
                        &[
                            file_path.to_string_lossy().to_string(),
                            "-o".to_string(),
                            output_binary.to_string_lossy().to_string(),
                        ],
                    );

                    if let Ok(result) = compile_status {
                        if result.ok {
                            runner.run_command(output_binary.to_string_lossy().as_ref(), &[])
                        } else {
                            Ok(result)
                        }
                    } else {
                        compile_status
                    }
                } else {
                    Ok(ActionResult::error(
                        "UnsupportedLanguage",
                        format!("Language '{}' is not supported", language),
                    ))
                }
            }
        };

        let res = match res {
            Ok(ok) => ok,
            Err(e) => ActionResult::error("ExecutionError", format!("{:#}", e)),
        };

        let timestamp = Utc::now().to_rfc3339();
        journal.record_result(&run_id, &res, &timestamp)?;
        results.push(res);
    }

    // Cloner les actions pour éviter le déplacement
    let actions = req.actions.clone();

    // Journaliser les actions de l'IA
    for action in &actions {
        if let Action::GenerateCode { language, code } = &action {
            let timestamp = Utc::now().to_rfc3339();
            journal.record_action(&run_id, action, &timestamp)?;

            // Sauvegarder le code généré dans l'espace IA
            let file_path = ai_workspace.join(format!("generated_{}.{}", timestamp, language));
            std::fs::write(&file_path, code)?;

            // Exécuter le code si nécessaire (par exemple, pour Python)
            if language == "python" {
                let res = runner.run_command("python3", &[file_path.to_string_lossy().to_string()]);
                let res = match res {
                    Ok(ok) => ok,
                    Err(e) => ActionResult::error("ExecutionError", format!("{:#}", e)),
                };

                let timestamp = Utc::now().to_rfc3339();
                journal.record_result(&run_id, &res, &timestamp)?;
                results.push(res);
            }
        }
    }

    // Optional scoring pass: summarize the session quality (unwrap penalties, clippy hints, etc.)
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

    let resp = Response {
        run_id,
        results,
        score,
    };
    print!("{}", serde_json::to_string_pretty(&resp)?);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Request {
    #[serde(default)]
    run_id: Option<String>,
    actions: Vec<Action>,
}

#[derive(Debug, Serialize)]
struct Response {
    run_id: String,
    results: Vec<ActionResult>,
    score: ScoreSummary,
}
