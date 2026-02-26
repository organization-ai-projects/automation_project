// projects/products/unstable/autonomy_orchestrator_ai/src/runtime.rs
use crate::artifacts::{NextActionsArtifact, save_next_actions};
use crate::checkpoint_store::load_checkpoint;
use crate::config_runtime::{
    derive_config_io_plan, first_non_binary_config_path, load_config_by_mode, save_config_by_mode,
};
use crate::domain::{
    BinaryInvocationSpec, CommandLineSpec, DeliveryOptions, ExecutionPolicy, GateInputs,
    OrchestratorCheckpoint, OrchestratorConfig, RunReport, Stage, StageExecutionStatus,
    TerminalState,
};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use crate::run_args::RunArgs;
use crate::runtime_diagnostics::print_runtime_diagnostics;
use crate::validation_invocation_parser::parse_validation_pending_invocations;
use crate::versioning::{RepoVersioningDelta, capture_repo_snapshot, compute_repo_delta};
use std::path::Path;
use std::process;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub fn run_orchestrator(args: RunArgs, raw_args: &[String]) -> ! {
    let code = run_orchestrator_internal(args, raw_args);
    process::exit(code);
}

fn run_orchestrator_internal(args: RunArgs, raw_args: &[String]) -> i32 {
    if args.execution_max_iterations == 0 {
        eprintln!("Invalid --execution-max-iterations value: must be >= 1");
        return 2;
    }
    if args.autonomous_max_runs == 0 {
        eprintln!("Invalid --autonomous-max-runs value: must be >= 1");
        return 2;
    }
    if args.autonomous_same_error_limit == 0 {
        eprintln!("Invalid --autonomous-same-error-limit value: must be >= 1");
        return 2;
    }

    if !args.autonomous_loop {
        return run_once(args, raw_args, false)
            .map(|outcome| outcome.exit_code)
            .unwrap_or_else(|(code, err)| {
                eprintln!("{err}");
                code
            });
    }

    run_autonomous_loop(args, raw_args).unwrap_or_else(|(code, err)| {
        eprintln!("{err}");
        code
    })
}

fn run_autonomous_loop(args: RunArgs, raw_args: &[String]) -> Result<i32, (i32, String)> {
    let started = Instant::now();
    let mut previous_failure_signature: Option<String> = None;
    let mut failure_streak = 0u32;
    let mut previous_noop_signature: Option<String> = None;
    let mut noop_streak = 0u32;
    let mut last_exit_code = 1;

    for run_index in 1..=args.autonomous_max_runs {
        let mut run_args = args.clone();
        if run_index > 1 {
            run_args.resume = true;
        }
        let outcome = run_once(run_args, raw_args, true)?;
        last_exit_code = outcome.exit_code;

        if outcome.exit_code == 0 {
            return Ok(0);
        }

        if started.elapsed().as_millis() > u128::from(args.autonomous_max_duration_ms) {
            println!(
                "Autonomous loop stopped after exceeding max duration ({} ms)",
                args.autonomous_max_duration_ms
            );
            return Ok(last_exit_code);
        }

        if let Some(signature) = failure_signature(&outcome.report) {
            if previous_failure_signature.as_deref() == Some(signature.as_str()) {
                failure_streak += 1;
            } else {
                previous_failure_signature = Some(signature);
                failure_streak = 1;
            }
            if failure_streak >= args.autonomous_same_error_limit {
                println!(
                    "Autonomous loop stopped: same failure signature repeated {} time(s)",
                    failure_streak
                );
                return Ok(last_exit_code);
            }
        }

        let noop_signature = build_noop_signature(
            &outcome.report,
            outcome.recommended_actions.as_ref(),
            outcome.versioning_delta.as_ref(),
        );
        if let Some(signature) = noop_signature {
            if previous_noop_signature.as_deref() == Some(signature.as_str()) {
                noop_streak += 1;
            } else {
                previous_noop_signature = Some(signature);
                noop_streak = 1;
            }
            if noop_streak >= 2 {
                println!("Autonomous loop stopped: no-op signature repeated");
                return Ok(last_exit_code);
            }
        }
    }

    println!(
        "Autonomous loop stopped: max runs reached ({})",
        args.autonomous_max_runs
    );
    Ok(last_exit_code)
}

struct RunOnceOutcome {
    report: RunReport,
    exit_code: i32,
    recommended_actions: Vec<String>,
    versioning_delta: Option<RepoVersioningDelta>,
}

fn run_once(
    args: RunArgs,
    raw_args: &[String],
    loop_mode: bool,
) -> Result<RunOnceOutcome, (i32, String)> {
    let config_io = derive_config_io_plan(&args).map_err(|err| (2, err))?;
    if args.ai_config_only_binary
        && let Some(path) = first_non_binary_config_path(&config_io)
    {
        return Err((
            2,
            format!(
                "AI binary-only mode forbids non-binary config path '{}'. Use .bin or no extension.",
                path.display()
            ),
        ));
    }

    let validation_pending_invocations =
        parse_validation_pending_invocations(raw_args).map_err(|err| (2, err))?;

    let checkpoint_path = args
        .checkpoint_path
        .clone()
        .unwrap_or_else(|| args.output_dir.join("orchestrator_checkpoint.json"));
    let cycle_memory_path = args
        .cycle_memory_path
        .clone()
        .unwrap_or_else(|| args.output_dir.join("orchestrator_cycle_memory.bin"));
    let next_actions_path = args
        .next_actions_path
        .clone()
        .unwrap_or_else(|| args.output_dir.join("next_actions.bin"));

    let resume_enabled = args.resume || loop_mode;
    let version_before = capture_repo_snapshot(&args.repo_root);

    let checkpoint = if resume_enabled {
        if checkpoint_path.exists() {
            Some(
                load_checkpoint(&checkpoint_path)
                    .map_err(|err| (1, format!("Failed to resume checkpoint: {err}")))?,
            )
        } else {
            None
        }
    } else {
        None
    };

    let repo_root_for_versioning = args.repo_root.clone();

    let gate_inputs = GateInputs {
        policy_status: args.policy_status.into(),
        ci_status: args.ci_status.into(),
        review_status: args.review_status.into(),
    };

    let planning_invocation = args.manager_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command,
            args: args.manager_args,
        },
        env: args.manager_env,
        timeout_ms: args.timeout_ms,
        expected_artifacts: args.manager_expected_artifacts,
    });

    let execution_invocation = args.executor_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Execution,
        command_line: CommandLineSpec {
            command,
            args: args.executor_args,
        },
        env: args.executor_env,
        timeout_ms: args.timeout_ms,
        expected_artifacts: args.executor_expected_artifacts,
    });

    let validation_invocation = args.reviewer_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Validation,
        command_line: CommandLineSpec {
            command,
            args: args.reviewer_args,
        },
        env: args.reviewer_env,
        timeout_ms: args.timeout_ms,
        expected_artifacts: args.reviewer_expected_artifacts,
    });

    let validation_invocations = validation_pending_invocations
        .into_iter()
        .map(|pending| BinaryInvocationSpec {
            stage: Stage::Validation,
            command_line: CommandLineSpec {
                command: pending.command,
                args: pending.args,
            },
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
        execution_policy: ExecutionPolicy {
            execution_max_iterations: args.execution_max_iterations,
            reviewer_remediation_max_cycles: args.reviewer_remediation_max_cycles,
        },
        timeout_ms: args.timeout_ms,
        repo_root: args.repo_root,
        planning_context_artifact: args.planning_context_artifact,
        validation_invocations,
        validation_from_planning_context: args.validation_from_planning_context,
        delivery_options,
        gate_inputs,
        checkpoint_path: Some(checkpoint_path.clone()),
        cycle_memory_path: Some(cycle_memory_path.clone()),
        next_actions_path: Some(next_actions_path.clone()),
    };

    if let Some(load_mode) = &config_io.load {
        config = load_config_by_mode(load_mode).map_err(|err| (1, err))?;
        config.checkpoint_path = Some(checkpoint_path.clone());
        config.cycle_memory_path = Some(cycle_memory_path.clone());
        config.next_actions_path = Some(next_actions_path.clone());
    }

    for save_mode in &config_io.saves {
        save_config_by_mode(&config, save_mode).map_err(|err| (1, err))?;
    }

    if args.verbose {
        print_runtime_diagnostics(
            &args.output_dir,
            resume_enabled,
            args.ai_config_only_binary,
            &config,
            &checkpoint_path,
            &config_io,
        );
    }

    let report = Orchestrator::new(config, checkpoint).execute();

    write_run_report(&report, &args.output_dir)
        .map_err(|err| (1, format!("Failed to write run report: {err}")))?;
    let recommended_actions = build_recommended_actions(&report);
    let version_after = capture_repo_snapshot(&repo_root_for_versioning);
    let versioning_delta = compute_repo_delta(version_before.as_ref(), version_after.as_ref());
    persist_next_actions(
        &report,
        &recommended_actions,
        versioning_delta.as_ref(),
        &next_actions_path,
    )
    .map_err(|err| (1, format!("Failed to write next actions artifact: {err}")))?;

    let report_path = args.output_dir.join("orchestrator_run_report.json");
    println!("Run report: {}", report_path.display());
    println!("Terminal state: {:?}", report.terminal_state);

    Ok(RunOnceOutcome {
        exit_code: terminal_state_exit_code(report.terminal_state),
        report,
        recommended_actions,
        versioning_delta,
    })
}

fn persist_next_actions(
    report: &RunReport,
    recommended_actions: &[String],
    versioning_delta: Option<&RepoVersioningDelta>,
    path: &Path,
) -> Result<(), String> {
    let artifact = NextActionsArtifact {
        run_id: report.run_id.clone(),
        terminal_state: report.terminal_state,
        blocked_reason_codes: report.blocked_reason_codes.clone(),
        reviewer_next_steps: report.reviewer_next_steps.clone(),
        recommended_actions: recommended_actions.to_vec(),
        versioning_delta: versioning_delta.cloned(),
        generated_at_unix_secs: unix_timestamp_secs(),
    };
    save_next_actions(path, &artifact)
}

fn build_recommended_actions(report: &RunReport) -> Vec<String> {
    if !report.reviewer_next_steps.is_empty() {
        return report.reviewer_next_steps.clone();
    }
    let mut actions = Vec::new();
    for code in &report.blocked_reason_codes {
        let action = match code.as_str() {
            "GATE_POLICY_DENIED_OR_UNKNOWN" => {
                "Resolve policy gate: enforce allow signal before rerun".to_string()
            }
            "GATE_CI_NOT_SUCCESS" => "Resolve CI gate: ensure CI status is success".to_string(),
            "GATE_REVIEW_NOT_APPROVED" => {
                "Resolve review gate: obtain approved review status".to_string()
            }
            other => format!("Resolve blocked reason: {other}"),
        };
        if !actions.contains(&action) {
            actions.push(action);
        }
    }
    if actions.is_empty() && report.terminal_state == Some(TerminalState::Failed) {
        actions.push("Inspect failed stage execution and remediate before rerun".to_string());
    }
    actions
}

fn failure_signature(report: &RunReport) -> Option<String> {
    if report.terminal_state == Some(TerminalState::Done) {
        return None;
    }
    let failure_commands = report
        .stage_executions
        .iter()
        .filter(|entry| {
            matches!(
                entry.status,
                StageExecutionStatus::Failed
                    | StageExecutionStatus::Timeout
                    | StageExecutionStatus::SpawnFailed
                    | StageExecutionStatus::ArtifactMissing
            )
        })
        .map(|entry| entry.command.clone())
        .collect::<Vec<_>>()
        .join("|");
    Some(format!(
        "{:?}|{}|{}",
        report.terminal_state,
        report.blocked_reason_codes.join("|"),
        failure_commands
    ))
}

fn build_noop_signature(
    report: &RunReport,
    recommended_actions: &[String],
    versioning_delta: Option<&RepoVersioningDelta>,
) -> Option<String> {
    let failure = failure_signature(report)?;
    let delta = versioning_delta?;
    if delta.worktree_changed {
        return None;
    }
    Some(format!(
        "{failure}|{}|{}",
        delta.touched_files.join("|"),
        recommended_actions.join("|")
    ))
}

fn terminal_state_exit_code(state: Option<TerminalState>) -> i32 {
    match state {
        Some(TerminalState::Done) => 0,
        Some(TerminalState::Blocked) => 3,
        Some(TerminalState::Timeout) => 124,
        Some(TerminalState::Failed) | None => 1,
    }
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
