// projects/products/unstable/autonomy_orchestrator_ai/src/main.rs

mod binary_runner;
mod checkpoint_store;
mod domain;
mod fixture;
mod linked_stack;
mod orchestrator;
mod output_writer;
mod repo_context_artifact;

use crate::checkpoint_store::load_checkpoint;
use crate::domain::{
    BinaryInvocationSpec, CiGateStatus, DeliveryOptions, GateInputs, OrchestratorCheckpoint,
    OrchestratorConfig, PolicyGateStatus, ReviewGateStatus, Stage, TerminalState,
};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct PendingValidationInvocation {
    command: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

#[derive(Parser, Debug)]
#[command(name = "autonomy_orchestrator_ai")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    run: RunArgs,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fixture {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    LinkedStack {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    ConfigValidate(ConfigValidateArgs),
    ConfigCanonicalize(ConfigCanonicalizeArgs),
}

#[derive(Args, Debug)]
struct ConfigValidateArgs {
    config_path: PathBuf,
    #[arg(long)]
    ai_config_only_binary: bool,
}

#[derive(Args, Debug)]
struct ConfigCanonicalizeArgs {
    input_config: PathBuf,
    output_bin_config: PathBuf,
    #[arg(long)]
    ai_config_only_binary: bool,
}

#[derive(Args, Debug)]
struct RunArgs {
    #[arg(default_value = "./out")]
    output_dir: PathBuf,

    #[arg(long)]
    simulate_blocked: bool,
    #[arg(long)]
    resume: bool,
    #[arg(long, default_value_t = 30_000)]
    timeout_ms: u64,

    #[arg(long, value_enum, default_value_t = CliPolicyStatus::Unknown)]
    policy_status: CliPolicyStatus,
    #[arg(long, value_enum, default_value_t = CliCiStatus::Missing)]
    ci_status: CliCiStatus,
    #[arg(long, value_enum, default_value_t = CliReviewStatus::Missing)]
    review_status: CliReviewStatus,

    #[arg(long)]
    checkpoint_path: Option<PathBuf>,

    #[arg(long)]
    manager_bin: Option<String>,
    #[arg(long = "manager-arg", action = ArgAction::Append)]
    manager_args: Vec<String>,
    #[arg(long = "manager-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    manager_env: Vec<(String, String)>,
    #[arg(long = "manager-expected-artifact", action = ArgAction::Append)]
    manager_expected_artifacts: Vec<String>,

    #[arg(long)]
    executor_bin: Option<String>,
    #[arg(long = "executor-arg", action = ArgAction::Append)]
    executor_args: Vec<String>,
    #[arg(long = "executor-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    executor_env: Vec<(String, String)>,
    #[arg(long = "executor-expected-artifact", action = ArgAction::Append)]
    executor_expected_artifacts: Vec<String>,

    #[arg(long, default_value_t = 1)]
    execution_max_iterations: u32,
    #[arg(long, default_value_t = 0)]
    reviewer_remediation_max_cycles: u32,

    #[arg(long)]
    reviewer_bin: Option<String>,
    #[arg(long = "reviewer-arg", action = ArgAction::Append)]
    reviewer_args: Vec<String>,
    #[arg(long = "reviewer-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    reviewer_env: Vec<(String, String)>,
    #[arg(long = "reviewer-expected-artifact", action = ArgAction::Append)]
    reviewer_expected_artifacts: Vec<String>,

    // Parsed manually from raw argv to preserve "binds to last --validation-bin" semantics.
    #[arg(long = "validation-bin", action = ArgAction::Append)]
    _validation_bins: Vec<String>,
    #[arg(long = "validation-arg", action = ArgAction::Append)]
    _validation_args: Vec<String>,
    #[arg(long = "validation-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    _validation_env: Vec<(String, String)>,

    #[arg(long)]
    validation_from_planning_context: bool,

    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
    #[arg(long)]
    planning_context_artifact: Option<PathBuf>,

    #[arg(long)]
    delivery_enabled: bool,
    #[arg(long)]
    delivery_dry_run: bool,
    #[arg(long)]
    delivery_branch: Option<String>,
    #[arg(long)]
    delivery_commit_message: Option<String>,
    #[arg(long)]
    delivery_pr_enabled: bool,
    #[arg(long)]
    delivery_pr_number: Option<String>,
    #[arg(long)]
    delivery_pr_base: Option<String>,
    #[arg(long)]
    delivery_pr_title: Option<String>,
    #[arg(long)]
    delivery_pr_body: Option<String>,

    #[arg(long)]
    config_save_ron: Option<PathBuf>,
    #[arg(long)]
    config_save_bin: Option<PathBuf>,
    #[arg(long)]
    config_save_json: Option<PathBuf>,
    #[arg(long = "config-save")]
    config_save_auto: Option<PathBuf>,

    #[arg(long)]
    config_load_ron: Option<PathBuf>,
    #[arg(long)]
    config_load_bin: Option<PathBuf>,
    #[arg(long)]
    config_load_json: Option<PathBuf>,
    #[arg(long = "config-load")]
    config_load_auto: Option<PathBuf>,

    #[arg(long)]
    ai_config_only_binary: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliPolicyStatus {
    Allow,
    Deny,
    Unknown,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliCiStatus {
    Success,
    Pending,
    Failure,
    Missing,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliReviewStatus {
    Approved,
    #[value(name = "changes_requested")]
    ChangesRequested,
    Missing,
}

impl From<CliPolicyStatus> for PolicyGateStatus {
    fn from(value: CliPolicyStatus) -> Self {
        match value {
            CliPolicyStatus::Allow => Self::Allow,
            CliPolicyStatus::Deny => Self::Deny,
            CliPolicyStatus::Unknown => Self::Unknown,
        }
    }
}

impl From<CliCiStatus> for CiGateStatus {
    fn from(value: CliCiStatus) -> Self {
        match value {
            CliCiStatus::Success => Self::Success,
            CliCiStatus::Pending => Self::Pending,
            CliCiStatus::Failure => Self::Failure,
            CliCiStatus::Missing => Self::Missing,
        }
    }
}

impl From<CliReviewStatus> for ReviewGateStatus {
    fn from(value: CliReviewStatus) -> Self {
        match value {
            CliReviewStatus::Approved => Self::Approved,
            CliReviewStatus::ChangesRequested => Self::ChangesRequested,
            CliReviewStatus::Missing => Self::Missing,
        }
    }
}

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Fixture { args }) => fixture::run(&args),
        Some(Commands::LinkedStack { args }) => match linked_stack::run(&args) {
            Ok(()) => process::exit(0),
            Err(error) => {
                eprintln!("{error}");
                process::exit(1);
            }
        },
        Some(Commands::ConfigValidate(args)) => run_config_validate(args),
        Some(Commands::ConfigCanonicalize(args)) => run_config_canonicalize(args),
        None => run_orchestrator(cli.run, &raw_args),
    }
}

fn run_orchestrator(args: RunArgs, raw_args: &[String]) -> ! {
    if args.execution_max_iterations == 0 {
        eprintln!("Invalid --execution-max-iterations value: must be >= 1");
        process::exit(2);
    }

    let load_modes_selected = [
        args.config_load_ron.is_some(),
        args.config_load_bin.is_some(),
        args.config_load_json.is_some(),
        args.config_load_auto.is_some(),
    ]
    .into_iter()
    .filter(|selected| *selected)
    .count();
    if load_modes_selected > 1 {
        eprintln!(
            "Only one config load mode is allowed: choose exactly one of --config-load, --config-load-ron, --config-load-bin, --config-load-json"
        );
        process::exit(2);
    }

    if args.config_save_auto.is_some()
        && (args.config_save_ron.is_some()
            || args.config_save_bin.is_some()
            || args.config_save_json.is_some())
    {
        eprintln!(
            "When --config-save is used, do not combine it with --config-save-ron/--config-save-bin/--config-save-json"
        );
        process::exit(2);
    }

    if args.ai_config_only_binary {
        for path in [
            args.config_load_auto.as_ref(),
            args.config_load_ron.as_ref(),
            args.config_load_bin.as_ref(),
            args.config_load_json.as_ref(),
            args.config_save_auto.as_ref(),
            args.config_save_ron.as_ref(),
            args.config_save_bin.as_ref(),
            args.config_save_json.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            if !is_binary_config_path(path) {
                eprintln!(
                    "AI binary-only mode forbids non-binary config path '{}'. Use .bin or no extension.",
                    path.display()
                );
                process::exit(2);
            }
        }
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

    if let Some(path) = &args.config_load_auto {
        config = OrchestratorConfig::load_auto(path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        config.checkpoint_path = Some(checkpoint_path.clone());
    } else if let Some(path) = &args.config_load_ron {
        config = OrchestratorConfig::load_ron(path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        config.checkpoint_path = Some(checkpoint_path.clone());
    } else if let Some(path) = &args.config_load_bin {
        config = OrchestratorConfig::load_bin(path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        config.checkpoint_path = Some(checkpoint_path.clone());
    } else if let Some(path) = &args.config_load_json {
        config = OrchestratorConfig::load_json(path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        config.checkpoint_path = Some(checkpoint_path.clone());
    }

    if let Some(path) = &args.config_save_ron
        && let Err(err) = config.save_ron(path)
    {
        eprintln!("{err}");
        process::exit(1);
    }
    if let Some(path) = &args.config_save_bin
        && let Err(err) = config.save_bin(path)
    {
        eprintln!("{err}");
        process::exit(1);
    }
    if let Some(path) = &args.config_save_json
        && let Err(err) = config.save_json(path)
    {
        eprintln!("{err}");
        process::exit(1);
    }
    if let Some(path) = &args.config_save_auto
        && let Err(err) = config.save_auto(path)
    {
        eprintln!("{err}");
        process::exit(1);
    }

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
    println!("Config load AUTO: {}", args.config_load_auto.is_some());
    println!("Config load RON: {}", args.config_load_ron.is_some());
    println!("Config load BIN: {}", args.config_load_bin.is_some());
    println!("Config load JSON: {}", args.config_load_json.is_some());
    println!("Config save AUTO: {}", args.config_save_auto.is_some());
    println!("Config save RON: {}", args.config_save_ron.is_some());
    println!("Config save BIN: {}", args.config_save_bin.is_some());
    println!("Config save JSON: {}", args.config_save_json.is_some());
    println!("AI config only binary: {}", args.ai_config_only_binary);
    println!();

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

fn parse_env_pair_cli(raw: &str) -> Result<(String, String), String> {
    let mut split = raw.splitn(2, '=');
    let key = split.next().unwrap_or_default().trim();
    let value = split.next();
    if key.is_empty() || value.is_none() {
        return Err(format!("Invalid env pair '{}', expected KEY=VALUE", raw));
    }
    Ok((key.to_string(), value.unwrap_or_default().to_string()))
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

fn run_config_validate(args: ConfigValidateArgs) -> ! {
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

fn run_config_canonicalize(args: ConfigCanonicalizeArgs) -> ! {
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
