use crate::checkpoint_store::load_checkpoint;
use crate::config_runtime::{
    derive_config_io_plan, first_non_binary_config_path, load_config_by_mode, save_config_by_mode,
};
use crate::domain::{
    BinaryInvocationSpec, DeliveryOptions, GateInputs, OrchestratorCheckpoint, OrchestratorConfig,
    Stage, TerminalState,
};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use crate::run_args::RunArgs;
use crate::runtime_diagnostics::print_runtime_diagnostics;
use crate::validation_invocation_parser::parse_validation_pending_invocations;
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
        print_runtime_diagnostics(
            &args.output_dir,
            args.resume,
            args.ai_config_only_binary,
            &config,
            &checkpoint_path,
            &config_io,
        );
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

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
