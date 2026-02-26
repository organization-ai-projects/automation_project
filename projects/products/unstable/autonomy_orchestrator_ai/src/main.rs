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
    BinaryInvocationSpec, CiGateStatus, GateInputs, OrchestratorCheckpoint, PolicyGateStatus,
    ReviewGateStatus, Stage, TerminalState,
};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct PendingValidationInvocation {
    command: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    if matches!(raw_args.first().map(String::as_str), Some("fixture")) {
        fixture::run(&raw_args[1..]);
    }
    if matches!(raw_args.first().map(String::as_str), Some("linked-stack")) {
        match linked_stack::run(&raw_args[1..]) {
            Ok(()) => process::exit(0),
            Err(error) => {
                eprintln!("{error}");
                process::exit(1);
            }
        }
    }

    let mut output_dir = PathBuf::from("./out");
    let mut simulate_blocked = false;
    let mut resume = false;
    let mut timeout_ms: u64 = 30_000;
    let mut policy_status = PolicyGateStatus::Unknown;
    let mut ci_status = CiGateStatus::Missing;
    let mut review_status = ReviewGateStatus::Missing;
    let mut checkpoint_path_override: Option<PathBuf> = None;
    let mut manager_bin: Option<String> = None;
    let mut manager_args: Vec<String> = Vec::new();
    let mut manager_env: Vec<(String, String)> = Vec::new();
    let mut manager_expected_artifacts: Vec<String> = Vec::new();
    let mut executor_bin: Option<String> = None;
    let mut executor_args: Vec<String> = Vec::new();
    let mut executor_env: Vec<(String, String)> = Vec::new();
    let mut executor_expected_artifacts: Vec<String> = Vec::new();
    let mut execution_max_iterations: u32 = 1;
    let mut reviewer_remediation_max_cycles: u32 = 0;
    let mut reviewer_bin: Option<String> = None;
    let mut reviewer_args: Vec<String> = Vec::new();
    let mut reviewer_env: Vec<(String, String)> = Vec::new();
    let mut reviewer_expected_artifacts: Vec<String> = Vec::new();
    let mut validation_pending_invocations: Vec<PendingValidationInvocation> = Vec::new();
    let mut validation_from_planning_context = false;
    let mut repo_root = PathBuf::from(".");
    let mut planning_context_artifact: Option<PathBuf> = None;

    let args = raw_args;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--simulate-blocked" => {
                simulate_blocked = true;
                i += 1;
            }
            "--resume" => {
                resume = true;
                i += 1;
            }
            "--timeout-ms" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                timeout_ms = args[i + 1].parse::<u64>().unwrap_or_else(|_| {
                    eprintln!("Invalid --timeout-ms value: {}", args[i + 1]);
                    process::exit(2);
                });
                i += 2;
            }
            "--policy-status" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                policy_status = parse_policy_status(&args[i + 1]);
                i += 2;
            }
            "--ci-status" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                ci_status = parse_ci_status(&args[i + 1]);
                i += 2;
            }
            "--review-status" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                review_status = parse_review_status(&args[i + 1]);
                i += 2;
            }
            "--checkpoint-path" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                checkpoint_path_override = Some(PathBuf::from(args[i + 1].clone()));
                i += 2;
            }
            "--manager-bin" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_bin = Some(args[i + 1].clone());
                i += 2;
            }
            "--manager-arg" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_args.push(args[i + 1].clone());
                i += 2;
            }
            "--manager-env" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_env.push(parse_env_pair(&args[i + 1]));
                i += 2;
            }
            "--manager-expected-artifact" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_expected_artifacts.push(args[i + 1].clone());
                i += 2;
            }
            "--executor-bin" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_bin = Some(args[i + 1].clone());
                i += 2;
            }
            "--executor-arg" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_args.push(args[i + 1].clone());
                i += 2;
            }
            "--executor-env" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_env.push(parse_env_pair(&args[i + 1]));
                i += 2;
            }
            "--executor-expected-artifact" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_expected_artifacts.push(args[i + 1].clone());
                i += 2;
            }
            "--execution-max-iterations" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                execution_max_iterations = args[i + 1].parse::<u32>().unwrap_or_else(|_| {
                    eprintln!("Invalid --execution-max-iterations value: {}", args[i + 1]);
                    process::exit(2);
                });
                if execution_max_iterations == 0 {
                    eprintln!("Invalid --execution-max-iterations value: must be >= 1");
                    process::exit(2);
                }
                i += 2;
            }
            "--reviewer-remediation-max-cycles" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                reviewer_remediation_max_cycles = args[i + 1].parse::<u32>().unwrap_or_else(|_| {
                    eprintln!(
                        "Invalid --reviewer-remediation-max-cycles value: {}",
                        args[i + 1]
                    );
                    process::exit(2);
                });
                i += 2;
            }
            "--reviewer-bin" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                reviewer_bin = Some(args[i + 1].clone());
                i += 2;
            }
            "--reviewer-arg" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                reviewer_args.push(args[i + 1].clone());
                i += 2;
            }
            "--reviewer-env" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                reviewer_env.push(parse_env_pair(&args[i + 1]));
                i += 2;
            }
            "--reviewer-expected-artifact" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                reviewer_expected_artifacts.push(args[i + 1].clone());
                i += 2;
            }
            "--validation-bin" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                validation_pending_invocations.push(PendingValidationInvocation {
                    command: args[i + 1].clone(),
                    args: Vec::new(),
                    env: Vec::new(),
                });
                i += 2;
            }
            "--validation-arg" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                let Some(last) = validation_pending_invocations.last_mut() else {
                    eprintln!("--validation-arg requires a preceding --validation-bin");
                    process::exit(2);
                };
                last.args.push(args[i + 1].clone());
                i += 2;
            }
            "--validation-env" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                let Some(last) = validation_pending_invocations.last_mut() else {
                    eprintln!("--validation-env requires a preceding --validation-bin");
                    process::exit(2);
                };
                last.env.push(parse_env_pair(&args[i + 1]));
                i += 2;
            }
            "--validation-from-planning-context" => {
                validation_from_planning_context = true;
                i += 1;
            }
            "--repo-root" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                repo_root = PathBuf::from(args[i + 1].clone());
                i += 2;
            }
            "--planning-context-artifact" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                planning_context_artifact = Some(PathBuf::from(args[i + 1].clone()));
                i += 2;
            }
            val if val.starts_with("--") => {
                eprintln!("Unknown option: {val}");
                usage_and_exit();
            }
            val => {
                output_dir = PathBuf::from(val);
                i += 1;
            }
        }
    }

    let checkpoint_path = checkpoint_path_override
        .clone()
        .unwrap_or_else(|| output_dir.join("orchestrator_checkpoint.json"));
    let checkpoint = if resume {
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
    let run_id = checkpoint
        .as_ref()
        .map(|cp: &OrchestratorCheckpoint| cp.run_id.clone())
        .unwrap_or_else(|| format!("run_{}", unix_timestamp_secs()));
    let gate_inputs = GateInputs {
        policy_status,
        ci_status,
        review_status,
    };
    let planning_invocation = manager_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Planning,
        command,
        args: manager_args,
        env: manager_env,
        timeout_ms,
        expected_artifacts: manager_expected_artifacts,
    });
    let execution_invocation = executor_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Execution,
        command,
        args: executor_args,
        env: executor_env,
        timeout_ms,
        expected_artifacts: executor_expected_artifacts,
    });
    let validation_invocation = reviewer_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Validation,
        command,
        args: reviewer_args,
        env: reviewer_env,
        timeout_ms,
        expected_artifacts: reviewer_expected_artifacts,
    });
    let validation_invocations = validation_pending_invocations
        .into_iter()
        .map(|pending| BinaryInvocationSpec {
            stage: Stage::Validation,
            command: pending.command,
            args: pending.args,
            env: pending.env,
            timeout_ms,
            expected_artifacts: Vec::new(),
        })
        .collect::<Vec<_>>();

    println!("Autonomy Orchestrator AI V0");
    println!("Run ID: {}", run_id);
    println!("Output: {}", output_dir.display());
    println!("Resume: {}", resume);
    println!("Checkpoint path: {}", checkpoint_path.display());
    println!("Simulate blocked: {}", simulate_blocked);
    println!("Timeout ms: {}", timeout_ms);
    println!("Policy status: {:?}", gate_inputs.policy_status);
    println!("CI status: {:?}", gate_inputs.ci_status);
    println!("Review status: {:?}", gate_inputs.review_status);
    println!("Repo root: {}", repo_root.display());
    println!("Execution max iterations: {}", execution_max_iterations);
    println!(
        "Reviewer remediation max cycles: {}",
        reviewer_remediation_max_cycles
    );
    println!(
        "Planning invocation configured: {}",
        planning_invocation.is_some()
    );
    println!(
        "Planning context artifact configured: {}",
        planning_context_artifact.is_some()
    );
    println!(
        "Execution invocation configured: {}",
        execution_invocation.is_some()
    );
    println!(
        "Validation invocation configured: {}",
        validation_invocation.is_some()
    );
    println!(
        "Validation commands configured: {}",
        validation_invocations.len()
    );
    println!(
        "Validation from planning context: {}",
        validation_from_planning_context
    );
    println!();

    let report = Orchestrator::new(
        run_id.clone(),
        simulate_blocked,
        planning_invocation,
        execution_invocation,
        validation_invocation,
        execution_max_iterations,
        reviewer_remediation_max_cycles,
        timeout_ms,
        repo_root,
        planning_context_artifact,
        validation_invocations,
        validation_from_planning_context,
        gate_inputs,
        checkpoint,
        Some(checkpoint_path),
    )
    .execute();

    if let Err(err) = write_run_report(&report, &output_dir) {
        eprintln!("Failed to write run report: {err}");
        process::exit(1);
    }

    let report_path = output_dir.join("orchestrator_run_report.json");
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

fn usage_and_exit() -> ! {
    eprintln!("Usage:");
    eprintln!("  autonomy_orchestrator_ai [output_dir] [options]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --resume");
    eprintln!("  --checkpoint-path <path>");
    eprintln!("  --simulate-blocked");
    eprintln!("  --timeout-ms <millis>");
    eprintln!("  --policy-status <allow|deny|unknown>");
    eprintln!("  --ci-status <success|pending|failure|missing>");
    eprintln!("  --review-status <approved|changes_requested|missing>");
    eprintln!("  --manager-bin <path>");
    eprintln!("  --manager-arg <value>                    (repeatable)");
    eprintln!("  --manager-env <KEY=VALUE>               (repeatable)");
    eprintln!("  --manager-expected-artifact <path>       (repeatable)");
    eprintln!("  --executor-bin <path>");
    eprintln!("  --executor-arg <value>                   (repeatable)");
    eprintln!("  --executor-env <KEY=VALUE>              (repeatable)");
    eprintln!("  --executor-expected-artifact <path>      (repeatable)");
    eprintln!("  --execution-max-iterations <count>       (default: 1)");
    eprintln!("  --reviewer-remediation-max-cycles <count> (default: 0)");
    eprintln!("  --reviewer-bin <path>");
    eprintln!("  --reviewer-arg <value>                   (repeatable)");
    eprintln!("  --reviewer-env <KEY=VALUE>               (repeatable)");
    eprintln!("  --reviewer-expected-artifact <path>      (repeatable)");
    eprintln!("  --validation-bin <path>                  (repeatable)");
    eprintln!(
        "  --validation-arg <value>                 (repeatable; binds to last --validation-bin)"
    );
    eprintln!(
        "  --validation-env <KEY=VALUE>             (repeatable; binds to last --validation-bin)"
    );
    eprintln!("  --validation-from-planning-context");
    eprintln!("  --repo-root <path>                       (default: .)");
    eprintln!("  --planning-context-artifact <path>");
    process::exit(2);
}

fn parse_env_pair(raw: &str) -> (String, String) {
    let mut split = raw.splitn(2, '=');
    let key = split.next().unwrap_or_default().trim();
    let value = split.next();
    if key.is_empty() || value.is_none() {
        eprintln!("Invalid env pair '{}', expected KEY=VALUE", raw);
        process::exit(2);
    }
    (key.to_string(), value.unwrap_or_default().to_string())
}

fn parse_policy_status(raw: &str) -> PolicyGateStatus {
    match raw.trim().to_ascii_lowercase().as_str() {
        "allow" => PolicyGateStatus::Allow,
        "deny" => PolicyGateStatus::Deny,
        "unknown" => PolicyGateStatus::Unknown,
        _ => {
            eprintln!(
                "Invalid --policy-status '{}', expected allow|deny|unknown",
                raw
            );
            process::exit(2);
        }
    }
}

fn parse_ci_status(raw: &str) -> CiGateStatus {
    match raw.trim().to_ascii_lowercase().as_str() {
        "success" => CiGateStatus::Success,
        "pending" => CiGateStatus::Pending,
        "failure" => CiGateStatus::Failure,
        "missing" => CiGateStatus::Missing,
        _ => {
            eprintln!(
                "Invalid --ci-status '{}', expected success|pending|failure|missing",
                raw
            );
            process::exit(2);
        }
    }
}

fn parse_review_status(raw: &str) -> ReviewGateStatus {
    match raw.trim().to_ascii_lowercase().as_str() {
        "approved" => ReviewGateStatus::Approved,
        "changes_requested" => ReviewGateStatus::ChangesRequested,
        "missing" => ReviewGateStatus::Missing,
        _ => {
            eprintln!(
                "Invalid --review-status '{}', expected approved|changes_requested|missing",
                raw
            );
            process::exit(2);
        }
    }
}
