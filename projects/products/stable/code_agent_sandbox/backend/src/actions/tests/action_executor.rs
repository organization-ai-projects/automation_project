use std::path::PathBuf;

use common_time::TimeSpan;

use crate::actions::{Action, execute_action};
use crate::command_runner::CommandRunner;
use crate::execution_context::ExecutionContext;
use crate::execution_paths::ExecutionPaths;
use crate::journal::Journal;
use crate::policies::{Policy, PolicyConfig};
use crate::runner_config::RunnerConfig;
use crate::sandbox_engine::{EngineConfig, EngineCtx};
use crate::sandbox_fs::SandboxFs;

fn make_policy(work_root: PathBuf) -> Policy {
    let mut engine_cfg = EngineConfig::new(TimeSpan::from_secs(1));
    engine_cfg.max_read_bytes = 8 * 1024;
    engine_cfg.max_write_bytes = 8 * 1024;
    engine_cfg.max_files_per_request = 64;

    let context = ExecutionContext {
        source_repo_root: work_root.clone(),
        paths: ExecutionPaths::new(work_root.clone(), work_root.clone()),
    };

    let cfg = PolicyConfig {
        context,
        max_read_bytes: engine_cfg.max_read_bytes,
        max_write_bytes: engine_cfg.max_write_bytes,
        max_files_per_request: engine_cfg.max_files_per_request,
        forbid_globs: vec![],
        allow_write_globs: vec!["**".to_string()],
        allow_read_globs: vec!["**".to_string()],
    };

    Policy::new(cfg).expect("policy should build")
}

#[test]
fn execute_action_generate_code_writes_expected_file() {
    let run_dir = tempfile::tempdir().expect("temp dir");
    let journal_file = tempfile::NamedTempFile::new().expect("journal file");

    let policy = make_policy(run_dir.path().to_path_buf());
    let sfs = SandboxFs::new(policy.clone());
    let runner = CommandRunner::new(
        policy,
        RunnerConfig {
            allowed_cargo_subcommands: vec!["check".to_string()],
            cargo_path: "cargo".to_string(),
        },
    );

    let mut journal = Journal::new(journal_file.path().to_path_buf()).expect("journal");
    let mut ctx = EngineCtx {
        run_id: "run-test".to_string(),
        sfs,
        runner,
        journal: &mut journal,
    };

    let action = Action::GenerateCode {
        language: "rs".to_string(),
        code: "fn generated() {}".to_string(),
    };

    let result = execute_action(&mut ctx, &action, run_dir.path()).expect("execution result");
    assert!(result.ok);
    assert_eq!(result.kind, "CodeGenerated");

    let generated = run_dir.path().join("generated_code.rs");
    assert!(generated.exists());
}
