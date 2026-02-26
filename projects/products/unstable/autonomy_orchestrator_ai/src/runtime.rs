use crate::checkpoint_store::load_checkpoint;
use crate::configs::{
    ConfigCanonicalizeArgs, ConfigIoPlan, ConfigLoadMode, ConfigSaveMode, ConfigValidateArgs,
};
use crate::domain::{
    BinaryInvocationSpec, DeliveryOptions, GateInputs, OrchestratorCheckpoint, OrchestratorConfig,
    Stage, TerminalState,
};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use crate::pending_validation_invocation::PendingValidationInvocation;
use crate::run_args::RunArgs;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_orchestrator(args: RunArgs, raw_args: &[String]) -> ! {
    if args.execution_max_iterations == 0 {
        eprintln!("Invalid --execution-max-iterations value: must be >= 1");
        process::exit(2);
    }

    let config_io = derive_config_io_plan(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(2);
    });
    if args.ai_config_only_binary
        && let Some(path) = first_non_binary_config_path(&config_io)
    {
        eprintln!(
            "AI binary-only mode forbids non-binary config path '{}'. Use .bin or no extension.",
            path.display()
        );
        process::exit(2);
    }

    let validation_pending_invocations = parse_validation_pending_invocations(raw_args)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(2);
        });

    let checkpoint_path = args
        .checkpoint_path
        .clone()
        .unwrap_or_else(|| args.output_dir.join("orchestrator_checkpoint.json"));

    let checkpoint = if args.resume {
        match load_checkpoint(&checkpoint_path) {
            Ok(cp) => Some(cp),
            Err(err) => {
                eprintln!("Failed to resume checkpoint: {err}");
                process::exit(1);
            }
        }
    } else {
        None
    };

    let gate_inputs = GateInputs {
        policy_status: args.policy_status.into(),
        ci_status: args.ci_status.into(),
        review_status: args.review_status.into(),
    };

    let planning_invocation = args.manager_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Planning,
        command,
        args: args.manager_args,
        env: args.manager_env,
        timeout_ms: args.timeout_ms,
        expected_artifacts: args.manager_expected_artifacts,
    });

    let execution_invocation = args.executor_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Execution,
        command,
        args: args.executor_args,
        env: args.executor_env,
        timeout_ms: args.timeout_ms,
        expected_artifacts: args.executor_expected_artifacts,
    });

    let validation_invocation = args.reviewer_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Validation,
        command,
        args: args.reviewer_args,
        env: args.reviewer_env,
        timeout_ms: args.timeout_ms,
        expected_artifacts: args.reviewer_expected_artifacts,
    });

    let validation_invocations = validation_pending_invocations
        .into_iter()
        .map(|pending| BinaryInvocationSpec {
            stage: Stage::Validation,
            command: pending.command,
            args: pending.args,
            env: pending.env,
            timeout_ms: args.timeout_ms,
            expected_artifacts: Vec::new(),
        })
        .collect::<Vec<_>>();

    let run_id = checkpoint
        .as_ref()
        .map(|cp: &OrchestratorCheckpoint| cp.run_id.clone())
        .unwrap_or_else(|| format!("run_{}", unix_timestamp_secs()));

    let mut delivery_options = DeliveryOptions::disabled();
    delivery_options.enabled = args.delivery_enabled;
    delivery_options.dry_run = args.delivery_dry_run;
    delivery_options.branch = args.delivery_branch;
    delivery_options.commit_message = args.delivery_commit_message;
    delivery_options.pr_enabled = args.delivery_pr_enabled;
    delivery_options.pr_number = args.delivery_pr_number;
    delivery_options.pr_base = args.delivery_pr_base;
    delivery_options.pr_title = args.delivery_pr_title;
    delivery_options.pr_body = args.delivery_pr_body;

    let mut config = OrchestratorConfig {
        run_id,
        simulate_blocked: args.simulate_blocked,
        planning_invocation,
        execution_invocation,
        validation_invocation,
        execution_max_iterations: args.execution_max_iterations,
        reviewer_remediation_max_cycles: args.reviewer_remediation_max_cycles,
        timeout_ms: args.timeout_ms,
        repo_root: args.repo_root,
        planning_context_artifact: args.planning_context_artifact,
        validation_invocations,
        validation_from_planning_context: args.validation_from_planning_context,
        delivery_options,
        gate_inputs,
        checkpoint_path: Some(checkpoint_path.clone()),
    };

    if let Some(load_mode) = &config_io.load {
        config = load_config_by_mode(load_mode).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        config.checkpoint_path = Some(checkpoint_path.clone());
    }

    for save_mode in &config_io.saves {
        if let Err(err) = save_config_by_mode(&config, save_mode) {
            eprintln!("{err}");
            process::exit(1);
        }
    }

    if args.verbose {
        println!("Autonomy Orchestrator AI V0");
        println!("Run ID: {}", config.run_id);
        println!("Output: {}", args.output_dir.display());
        println!("Resume: {}", args.resume);
        println!("Checkpoint path: {}", checkpoint_path.display());
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
        println!("AI config only binary: {}", args.ai_config_only_binary);
        println!();
    }

    let report = Orchestrator::new(config, checkpoint).execute();

    if let Err(err) = write_run_report(&report, &args.output_dir) {
        eprintln!("Failed to write run report: {err}");
        process::exit(1);
    }

    let report_path = args.output_dir.join("orchestrator_run_report.json");
    println!("Run report: {}", report_path.display());
    println!("Terminal state: {:?}", report.terminal_state);

    match report.terminal_state {
        Some(TerminalState::Done) => process::exit(0),
        Some(TerminalState::Blocked) => process::exit(3),
        Some(TerminalState::Timeout) => process::exit(124),
        Some(TerminalState::Failed) | None => process::exit(1),
    }
}

fn parse_validation_pending_invocations(
    raw_args: &[String],
) -> Result<Vec<PendingValidationInvocation>, String> {
    let mut result: Vec<PendingValidationInvocation> = Vec::new();
    let mut i = 0usize;
    while i < raw_args.len() {
        match raw_args[i].as_str() {
            "--validation-bin" => {
                if i + 1 >= raw_args.len() {
                    return Err("--validation-bin requires a value".to_string());
                }
                result.push(PendingValidationInvocation {
                    command: raw_args[i + 1].clone(),
                    args: Vec::new(),
                    env: Vec::new(),
                });
                i += 2;
            }
            "--validation-arg" => {
                if i + 1 >= raw_args.len() {
                    return Err("--validation-arg requires a value".to_string());
                }
                let Some(last) = result.last_mut() else {
                    return Err(
                        "--validation-arg requires a preceding --validation-bin".to_string()
                    );
                };
                last.args.push(raw_args[i + 1].clone());
                i += 2;
            }
            "--validation-env" => {
                if i + 1 >= raw_args.len() {
                    return Err("--validation-env requires a value".to_string());
                }
                let Some(last) = result.last_mut() else {
                    return Err(
                        "--validation-env requires a preceding --validation-bin".to_string()
                    );
                };
                let env_pair = parse_env_pair_cli(&raw_args[i + 1])?;
                last.env.push(env_pair);
                i += 2;
            }
            _ => i += 1,
        }
    }
    Ok(result)
}

pub(crate) fn parse_env_pair_cli(raw: &str) -> Result<(String, String), String> {
    let mut split = raw.splitn(2, '=');
    let key = split.next().unwrap_or_default().trim();
    let value = split.next();
    if key.is_empty() || value.is_none() {
        return Err(format!("Invalid env pair '{}', expected KEY=VALUE", raw));
    }
    Ok((key.to_string(), value.unwrap_or_default().to_string()))
}

fn derive_config_io_plan(args: &RunArgs) -> Result<ConfigIoPlan, String> {
    let mut plan = ConfigIoPlan::default();

    let mut load_modes = Vec::new();
    if let Some(path) = &args.config_load_auto {
        load_modes.push(ConfigLoadMode::Auto(path.clone()));
    }
    if let Some(path) = &args.config_load_ron {
        load_modes.push(ConfigLoadMode::Ron(path.clone()));
    }
    if let Some(path) = &args.config_load_bin {
        load_modes.push(ConfigLoadMode::Bin(path.clone()));
    }
    if let Some(path) = &args.config_load_json {
        load_modes.push(ConfigLoadMode::Json(path.clone()));
    }
    if load_modes.len() > 1 {
        return Err(
            "Only one config load mode is allowed: choose exactly one of --config-load, --config-load-ron, --config-load-bin, --config-load-json"
                .to_string(),
        );
    }
    plan.load = load_modes.into_iter().next();

    if args.config_save_auto.is_some()
        && (args.config_save_ron.is_some()
            || args.config_save_bin.is_some()
            || args.config_save_json.is_some())
    {
        return Err(
            "When --config-save is used, do not combine it with --config-save-ron/--config-save-bin/--config-save-json"
                .to_string(),
        );
    }

    if let Some(path) = &args.config_save_auto {
        plan.saves.push(ConfigSaveMode::Auto(path.clone()));
    }
    if let Some(path) = &args.config_save_ron {
        plan.saves.push(ConfigSaveMode::Ron(path.clone()));
    }
    if let Some(path) = &args.config_save_bin {
        plan.saves.push(ConfigSaveMode::Bin(path.clone()));
    }
    if let Some(path) = &args.config_save_json {
        plan.saves.push(ConfigSaveMode::Json(path.clone()));
    }

    Ok(plan)
}

fn first_non_binary_config_path(plan: &ConfigIoPlan) -> Option<&Path> {
    if let Some(load) = &plan.load {
        let path = match load {
            ConfigLoadMode::Auto(path)
            | ConfigLoadMode::Ron(path)
            | ConfigLoadMode::Bin(path)
            | ConfigLoadMode::Json(path) => path,
        };
        if !is_binary_config_path(path) {
            return Some(path);
        }
    }

    for save in &plan.saves {
        let path = match save {
            ConfigSaveMode::Auto(path)
            | ConfigSaveMode::Ron(path)
            | ConfigSaveMode::Bin(path)
            | ConfigSaveMode::Json(path) => path,
        };
        if !is_binary_config_path(path) {
            return Some(path);
        }
    }

    None
}

fn load_config_by_mode(mode: &ConfigLoadMode) -> Result<OrchestratorConfig, String> {
    match mode {
        ConfigLoadMode::Auto(path) => OrchestratorConfig::load_auto(path),
        ConfigLoadMode::Ron(path) => OrchestratorConfig::load_ron(path),
        ConfigLoadMode::Bin(path) => OrchestratorConfig::load_bin(path),
        ConfigLoadMode::Json(path) => OrchestratorConfig::load_json(path),
    }
}

fn save_config_by_mode(config: &OrchestratorConfig, mode: &ConfigSaveMode) -> Result<(), String> {
    match mode {
        ConfigSaveMode::Auto(path) => config.save_auto(path),
        ConfigSaveMode::Ron(path) => config.save_ron(path),
        ConfigSaveMode::Bin(path) => config.save_bin(path),
        ConfigSaveMode::Json(path) => config.save_json(path),
    }
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn is_binary_config_path(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.trim().to_ascii_lowercase());
    matches!(ext.as_deref(), None | Some("bin"))
}

pub fn run_config_validate(args: ConfigValidateArgs) -> ! {
    if args.ai_config_only_binary && !is_binary_config_path(&args.config_path) {
        eprintln!(
            "AI binary-only mode forbids non-binary config path '{}'. Use .bin or no extension.",
            args.config_path.display()
        );
        process::exit(2);
    }

    let config = match OrchestratorConfig::load_auto(&args.config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let diagnostics = validate_orchestrator_config(&config);
    if diagnostics.is_empty() {
        println!("Config validation: OK ({})", args.config_path.display());
        process::exit(0);
    }

    eprintln!(
        "Config validation failed for '{}':",
        args.config_path.display()
    );
    for diag in diagnostics {
        eprintln!("- {diag}");
    }
    process::exit(3);
}

pub fn run_config_canonicalize(args: ConfigCanonicalizeArgs) -> ! {
    if args.ai_config_only_binary
        && (!is_binary_config_path(&args.input_config)
            || !is_binary_config_path(&args.output_bin_config))
    {
        eprintln!(
            "AI binary-only mode forbids non-binary config path(s). Use .bin or no extension."
        );
        process::exit(2);
    }

    let config = match OrchestratorConfig::load_auto(&args.input_config) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let diagnostics = validate_orchestrator_config(&config);
    if !diagnostics.is_empty() {
        eprintln!(
            "Config canonicalization blocked, input config is invalid '{}':",
            args.input_config.display()
        );
        for diag in diagnostics {
            eprintln!("- {diag}");
        }
        process::exit(3);
    }

    if let Err(err) = config.save_bin(&args.output_bin_config) {
        eprintln!("{err}");
        process::exit(1);
    }

    println!(
        "Canonical binary config written: {} -> {}",
        args.input_config.display(),
        args.output_bin_config.display()
    );
    process::exit(0);
}

fn validate_orchestrator_config(config: &OrchestratorConfig) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if config.timeout_ms == 0 {
        diagnostics.push(
            "timeout_ms must be > 0 (fix: set --timeout-ms <millis>, e.g. 30000)".to_string(),
        );
    }
    if config.execution_max_iterations == 0 {
        diagnostics.push(
            "execution_max_iterations must be >= 1 (fix: set --execution-max-iterations <count>)"
                .to_string(),
        );
    }
    if config.validation_from_planning_context && config.planning_context_artifact.is_none() {
        diagnostics.push(
            "validation_from_planning_context=true requires planning_context_artifact (fix: set --planning-context-artifact <path>)"
                .to_string(),
        );
    }
    if config.delivery_options.pr_enabled && !config.delivery_options.enabled {
        diagnostics.push(
            "delivery_options.pr_enabled=true requires delivery_options.enabled=true (fix: add --delivery-enabled)"
                .to_string(),
        );
    }
    diagnostics
}
