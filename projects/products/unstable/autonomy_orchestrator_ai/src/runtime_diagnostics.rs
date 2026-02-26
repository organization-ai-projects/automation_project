// projects/products/unstable/autonomy_orchestrator_ai/src/runtime_diagnostics.rs
use crate::configs::{ConfigIoPlan, ConfigLoadMode, ConfigSaveMode};
use crate::domain::OrchestratorConfig;
use std::path::Path;

pub fn print_runtime_diagnostics(
    output_dir: &Path,
    resume: bool,
    ai_config_only_binary: bool,
    config: &OrchestratorConfig,
    checkpoint_path: &Path,
    config_io: &ConfigIoPlan,
) {
    println!("Autonomy Orchestrator AI V0");
    println!("Run ID: {}", config.run_id);
    println!("Output: {}", output_dir.display());
    println!("Resume: {}", resume);
    println!("Checkpoint path: {}", checkpoint_path.display());
    println!(
        "Cycle memory path: {}",
        config
            .cycle_memory_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<disabled>".to_string())
    );
    println!(
        "Next actions path: {}",
        config
            .next_actions_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<disabled>".to_string())
    );
    println!("Simulate blocked: {}", config.simulate_blocked);
    println!("Timeout ms: {}", config.timeout_ms);
    println!("Policy status: {:?}", config.gate_inputs.policy_status);
    println!("CI status: {:?}", config.gate_inputs.ci_status);
    println!("Review status: {:?}", config.gate_inputs.review_status);
    println!("Repo root: {}", config.repo_root.display());
    println!(
        "Execution max iterations: {}",
        config.execution_max_iterations
    );
    println!(
        "Reviewer remediation max cycles: {}",
        config.reviewer_remediation_max_cycles
    );
    println!(
        "Planning invocation configured: {}",
        config.planning_invocation.is_some()
    );
    println!(
        "Planning context artifact configured: {}",
        config.planning_context_artifact.is_some()
    );
    println!(
        "Execution invocation configured: {}",
        config.execution_invocation.is_some()
    );
    println!(
        "Validation invocation configured: {}",
        config.validation_invocation.is_some()
    );
    println!(
        "Validation commands configured: {}",
        config.validation_invocations.len()
    );
    println!(
        "Validation from planning context: {}",
        config.validation_from_planning_context
    );
    println!("Delivery enabled: {}", config.delivery_options.enabled);
    println!("Delivery dry-run: {}", config.delivery_options.dry_run);
    println!(
        "Delivery PR enabled: {}",
        config.delivery_options.pr_enabled
    );
    println!(
        "Config load AUTO: {}",
        matches!(config_io.load.as_ref(), Some(ConfigLoadMode::Auto(_)))
    );
    println!(
        "Config load RON: {}",
        matches!(config_io.load.as_ref(), Some(ConfigLoadMode::Ron(_)))
    );
    println!(
        "Config load BIN: {}",
        matches!(config_io.load.as_ref(), Some(ConfigLoadMode::Bin(_)))
    );
    println!(
        "Config load JSON: {}",
        matches!(config_io.load.as_ref(), Some(ConfigLoadMode::Json(_)))
    );
    println!(
        "Config save AUTO: {}",
        config_io
            .saves
            .iter()
            .any(|mode| matches!(mode, ConfigSaveMode::Auto(_)))
    );
    println!(
        "Config save RON: {}",
        config_io
            .saves
            .iter()
            .any(|mode| matches!(mode, ConfigSaveMode::Ron(_)))
    );
    println!(
        "Config save BIN: {}",
        config_io
            .saves
            .iter()
            .any(|mode| matches!(mode, ConfigSaveMode::Bin(_)))
    );
    println!(
        "Config save JSON: {}",
        config_io
            .saves
            .iter()
            .any(|mode| matches!(mode, ConfigSaveMode::Json(_)))
    );
    println!("AI config only binary: {}", ai_config_only_binary);
    println!();
}
