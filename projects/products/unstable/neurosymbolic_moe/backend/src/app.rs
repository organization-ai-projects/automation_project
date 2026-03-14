//! projects/products/unstable/neurosymbolic_moe/backend/src/app.rs
use std::path::PathBuf;
use std::{
    collections::VecDeque,
    fs,
    fs::File,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    sync::{Mutex, OnceLock},
};

use crate::aggregator::AggregationStrategy;
use crate::apps::{
    DynError, SloThresholds, cmd_impl_check, run_concurrent_pipeline_checks_with_report,
    run_runtime_persistence_checks_with_report, run_training_and_cas_checks,
};
use crate::dataset_engine::DatasetTrainingBuildOptions;
use crate::echo_expert::EchoExpert;
use crate::moe_core::{self, ExpertCapability, Task, TaskPriority, TaskType};
use crate::orchestrator::{AutoImprovementPolicy, MoePipeline, MoePipelineBuilder};
use crate::policy_guard::{Policy, PolicyType};
use crate::router::HeuristicRouter;
use crate::trace_logger::TraceLogger;

const STATUS_COMPONENT_LINES: [&str; 13] = [
    "  moe_core          - Expert trait, Task model, ExecutionContext",
    "  expert_registry   - Pluggable expert registration",
    "  router            - Heuristic task routing",
    "  retrieval_engine  - RAG retrieval abstraction",
    "  memory_engine     - Short-term and long-term memory",
    "  buffer_manager    - Working and session buffers",
    "  dataset_engine    - Incremental trace-to-dataset pipeline",
    "  evaluation_engine - Expert and routing metrics",
    "  feedback_engine   - Execution feedback and corrections",
    "  aggregator        - Multi-expert output aggregation",
    "  policy_guard      - Output validation and policy checks",
    "  trace_logger      - Execution traces and telemetry",
    "  orchestrator      - Main orchestration pipeline",
];

type HealthSnapshot = (String, Vec<String>, String, Vec<String>);
type ServeMetricsOptions = (
    String,
    bool,
    u64,
    Vec<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    bool,
    Option<u64>,
);
const DEFAULT_ADMIN_AUDIT_LIMIT: usize = 50;
const MAX_ADMIN_AUDIT_LIMIT: usize = 1000;

fn audit_io_lock() -> &'static Mutex<()> {
    static AUDIT_IO_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    AUDIT_IO_LOCK.get_or_init(|| Mutex::new(()))
}

pub fn run() -> Result<(), DynError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "status" => cmd_status(),
        "trace" => cmd_trace(&args[2..]),
        "trainer-events" => cmd_trainer_events(&args[2..]),
        "impl-check" => cmd_impl_check(),
        "slo-gate" => cmd_slo_gate(&args[2..]),
        "metrics" => cmd_metrics(),
        "serve-metrics" => cmd_serve_metrics(&args[2..]),
        other => {
            tracing::error!("Unknown command: {other}");
            print_usage();
            Ok(())
        }
    }
}

fn cmd_run(args: &[String]) -> Result<(), DynError> {
    let (input, bootstrap_dataset_bundle_path) = parse_run_options(args)?;
    let mut pipeline = build_cli_pipeline();
    if let Some(bootstrap_path) = bootstrap_dataset_bundle_path.as_deref() {
        let payload = fs::read_to_string(bootstrap_path)?;
        let seeded = pipeline.bootstrap_initial_dataset_from_training_bundle_json(&payload)?;
        tracing::info!("Auto bootstrap dataset: {} seeded entries", seeded);
    }
    register_default_cli_experts(&mut pipeline)?;
    add_default_cli_policy(&mut pipeline);

    let task = Task::new("task-001", TaskType::CodeGeneration, input)
        .with_context("runtime")
        .with_priority(TaskPriority::High)
        .with_metadata("source", "cli");
    let task_kind = format!("{:?}", task.task_type());
    let task_priority = format!("{:?}", task.priority());
    let has_context = task.context().is_some();

    match pipeline.execute(task) {
        Ok(result) => log_run_success(&result, &task_kind, &task_priority, has_context),
        Err(e) => {
            tracing::error!("Pipeline execution failed: {e}");
        }
    }

    log_pipeline_runtime_summary(&pipeline);

    Ok(())
}

fn cmd_status() -> Result<(), DynError> {
    tracing::info!("neurosymbolic_moe platform v0.1.0");
    tracing::info!("");
    tracing::info!("Components:");
    for line in STATUS_COMPONENT_LINES {
        tracing::info!("{line}");
    }
    tracing::info!("");
    tracing::info!("Use `impl-check` to run the full component wiring smoke test.");
    Ok(())
}

fn cmd_trace(args: &[String]) -> Result<(), DynError> {
    let trace_path = if args.is_empty() {
        None
    } else {
        Some(PathBuf::from(&args[0]))
    };

    if let Some(path) = trace_path {
        tracing::info!("Trace output path: {}", path.display());
    }

    let task_id = moe_core::TaskId::new("trace-demo");
    let expert_id = moe_core::ExpertId::new("trace-expert");
    let mut logger = TraceLogger::new(8);

    logger.log_phase(
        task_id.clone(),
        moe_core::TracePhase::Routing,
        "route trace command".to_string(),
        Some(expert_id.clone()),
    );
    logger.log_phase(
        task_id.clone(),
        moe_core::TracePhase::Aggregation,
        "aggregate trace command".to_string(),
        Some(expert_id.clone()),
    );

    let by_task = logger.get_by_task(&task_id);
    let by_phase = logger.get_by_phase(&moe_core::TracePhase::Routing);
    let by_expert = logger.get_by_expert(&expert_id);
    let recent = logger.recent(1);

    tracing::info!(
        "Trace stats: total={} by_task={} by_phase={} by_expert={} recent={}",
        logger.count(),
        by_task.len(),
        by_phase.len(),
        by_expert.len(),
        recent.len()
    );

    logger.clear();
    tracing::info!("Trace logger cleared: {}", logger.count());
    Ok(())
}

fn cmd_trainer_events(args: &[String]) -> Result<(), DynError> {
    let mut pipeline = build_cli_pipeline();
    register_default_cli_experts(&mut pipeline)?;
    add_default_cli_policy(&mut pipeline);

    let command = args.first().map(String::as_str).unwrap_or("pending");
    match command {
        "pending" => {
            let pending = pipeline.trainer_trigger_events_pending();
            let listed = pipeline.trainer_trigger_events().len();
            let dead_letters = pipeline.trainer_trigger_dead_letter_events_total();
            tracing::info!(
                "trainer events pending={pending} listed={listed} dead_letter={dead_letters}"
            );
        }
        "list" => {
            tracing::info!(
                "trainer events list (count={})",
                pipeline.trainer_trigger_events().len()
            );
            for event in pipeline.trainer_trigger_events() {
                tracing::info!(
                    "event_id={} attempts={} last_attempted_at={:?}",
                    event.event_id,
                    event.delivery_attempts,
                    event.last_attempted_at
                );
            }
        }
        "dead" => {
            tracing::info!(
                "trainer dead-letter events list (count={})",
                pipeline.trainer_trigger_dead_letter_events().len()
            );
            for event in pipeline.trainer_trigger_dead_letter_events() {
                tracing::info!(
                    "dead_letter event_id={} attempts={} last_attempted_at={:?}",
                    event.event_id,
                    event.delivery_attempts,
                    event.last_attempted_at
                );
            }
        }
        "pop" => {
            let popped = pipeline.pop_next_trainer_trigger_event();
            tracing::info!("pop result: {}", popped.is_some());
        }
        "lease" => {
            let now_epoch_seconds = parse_u64_arg(args, 1, "now_epoch_seconds", 0)?;
            let leased = if args.get(2).is_some() {
                let min_retry_delay_seconds = parse_u64_arg(args, 2, "min_retry_delay_seconds", 0)?;
                pipeline
                    .lease_next_trainer_trigger_event(now_epoch_seconds, min_retry_delay_seconds)
            } else {
                pipeline.lease_next_trainer_trigger_event_with_policy(now_epoch_seconds)
            };
            tracing::info!("lease result: {}", leased.is_some());
        }
        "ack" => {
            let event_id = parse_u64_arg(args, 1, "event_id", 0)?;
            let acknowledged = pipeline.acknowledge_trainer_trigger_event(event_id);
            tracing::info!("ack result: {acknowledged}");
        }
        "fail" => {
            let event_id = parse_u64_arg(args, 1, "event_id", 0)?;
            let failed_at_epoch_seconds =
                parse_u64_arg(args, 2, "failed_at_epoch_seconds", event_id)?;
            let marked = pipeline
                .mark_trainer_trigger_event_delivery_failed(event_id, failed_at_epoch_seconds);
            tracing::info!("mark-failed result: {marked}");
        }
        "drain" => {
            let max_events = parse_usize_arg(args, 1, "max_events", 1)?;
            let drained = pipeline.drain_trainer_trigger_events(max_events);
            tracing::info!("drain result: {}", drained.len());
        }
        other => {
            return Err(
                std::io::Error::other(format!("unknown trainer-events command: {other}")).into(),
            );
        }
    }

    Ok(())
}

fn cli_input_or_default(args: &[String]) -> String {
    if args.is_empty() {
        "default task input".to_string()
    } else {
        args.join(" ")
    }
}

fn parse_u64_arg(args: &[String], idx: usize, name: &str, default: u64) -> Result<u64, DynError> {
    if let Some(raw) = args.get(idx) {
        raw.parse::<u64>()
            .map_err(|err| std::io::Error::other(format!("invalid {name}: {err}")).into())
    } else {
        Ok(default)
    }
}

fn parse_usize_arg(
    args: &[String],
    idx: usize,
    name: &str,
    default: usize,
) -> Result<usize, DynError> {
    if let Some(raw) = args.get(idx) {
        raw.parse::<usize>()
            .map_err(|err| std::io::Error::other(format!("invalid {name}: {err}")).into())
    } else {
        Ok(default)
    }
}

pub(crate) fn parse_run_options(args: &[String]) -> Result<(String, Option<String>), DynError> {
    let mut idx = 0_usize;
    let mut bootstrap_dataset_bundle_path = None;
    let mut input_parts = Vec::new();
    while idx < args.len() {
        let arg = &args[idx];
        if arg == "--bootstrap-dataset-bundle-json" {
            let value = args.get(idx + 1).ok_or_else(|| {
                std::io::Error::other("missing value for --bootstrap-dataset-bundle-json")
            })?;
            bootstrap_dataset_bundle_path = Some(value.to_string());
            idx += 2;
        } else {
            input_parts.push(arg.to_string());
            idx += 1;
        }
    }

    Ok((
        cli_input_or_default(&input_parts),
        bootstrap_dataset_bundle_path,
    ))
}

fn build_cli_pipeline() -> MoePipeline {
    let auto_improvement_policy = AutoImprovementPolicy::default()
        .with_min_dataset_entries(32)
        .with_min_success_ratio(0.65)
        .with_min_average_score(Some(0.55))
        .with_training_build_options(DatasetTrainingBuildOptions {
            generated_at: 0,
            validation_ratio: 0.2,
            min_score: Some(0.4),
            include_failure_entries: true,
            include_partial_entries: true,
            include_unknown_entries: false,
            require_correction_for_failure: false,
            split_seed: 7,
        });
    MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(3)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_auto_improvement_policy(auto_improvement_policy)
        .with_max_trainer_trigger_events(512)
        .with_max_trainer_trigger_dead_letter_events(512)
        .with_max_traces(1000)
        .build()
}

fn register_default_cli_experts(pipeline: &mut MoePipeline) -> Result<(), DynError> {
    let experts = [
        (
            "code_gen",
            "CodeGenerationExpert",
            vec![ExpertCapability::CodeGeneration],
        ),
        (
            "code_transform",
            "CodeTransformExpert",
            vec![ExpertCapability::CodeTransformation],
        ),
        (
            "validator",
            "ValidationExpert",
            vec![ExpertCapability::Validation],
        ),
    ];
    for (id, name, capabilities) in experts {
        pipeline.register_expert(Box::new(EchoExpert::new(id, name, capabilities)))?;
    }
    Ok(())
}

fn add_default_cli_policy(pipeline: &mut MoePipeline) {
    pipeline.add_policy(Policy {
        id: "length_check".to_string(),
        name: "Output Length Check".to_string(),
        description: "Ensures output is not too long".to_string(),
        policy_type: PolicyType::LengthLimit(10000),
        active: true,
    });
}

fn log_run_success(
    result: &crate::moe_core::AggregatedOutput,
    task_kind: &str,
    task_priority: &str,
    has_context: bool,
) {
    tracing::info!("Pipeline execution successful");
    if let Some(selected) = &result.selected_output {
        tracing::info!("Selected expert: {}", selected.expert_id.as_str());
        tracing::info!("Confidence: {:.2}", selected.confidence);
        tracing::info!("Output: {}", selected.content);
    }
    tracing::info!("Total outputs: {}", result.outputs.len());
    tracing::info!("Strategy: {}", result.strategy);
    tracing::info!("Task kind: {task_kind}, priority: {task_priority}, context: {has_context}");
}

fn log_pipeline_runtime_summary(pipeline: &MoePipeline) {
    tracing::info!(
        "\nExpert registry: {} experts registered",
        pipeline.registry().count()
    );
    tracing::info!("Trace log: {} entries", pipeline.trace_logger().count());
    tracing::info!(
        "Dataset store: {} entries",
        pipeline.dataset_store().count()
    );
    let auto = pipeline.auto_improvement_status();
    tracing::info!(
        "Auto improvement: runs={} bootstrap_entries={} last_included={}",
        auto.runs_total,
        auto.bootstrap_entries_total,
        auto.last_included_entries
    );
}

fn print_usage() {
    tracing::info!("neurosymbolic_moe - advanced modular MoE platform");
    tracing::info!("");
    tracing::info!("Commands:");
    tracing::info!(
        "  run [--bootstrap-dataset-bundle-json PATH] [input...]     Execute a task through the MoE pipeline"
    );
    tracing::info!("  status             Show platform component status");
    tracing::info!("  trace [path]       Inspect execution traces");
    tracing::info!("  trainer-events [pending|list|dead|pop|lease|ack|fail|drain] [args...]");
    tracing::info!("  impl-check         Execute full component wiring check");
    tracing::info!("  slo-gate [flags]   Fail-fast SLO gate for CI");
    tracing::info!("  metrics            Print Prometheus-compatible metrics snapshot");
    tracing::info!(
        "  serve-metrics [addr] [--once] [--cache-ttl-requests N] [--slo-profile-path PATH] [--admin-token TOKEN]"
    );
    tracing::info!(
        "                   [--slo-audit-path PATH] [--slo-audit-max-bytes N] [--disable-auto-rollback]"
    );
    tracing::info!(
        "                   Serves /metrics, /healthz, /readyz, /livez, /admin/slo-profile, /admin/slo-audit"
    );
    tracing::info!("SLO flags:");
    tracing::info!("  --runtime-min-successes N");
    tracing::info!("  --runtime-max-rejections N");
    tracing::info!("  --runtime-max-parse-failures N");
    tracing::info!("  --profile strict|balanced|exploratory");
    tracing::info!("  --concurrent-max-contention-rate F");
    tracing::info!("  --concurrent-max-timeout-rate F");
    tracing::info!("  --concurrent-min-successes N");
    tracing::info!("  --concurrent-max-rejections N");
    tracing::info!("  --concurrent-max-parse-failures N");
}

fn cmd_slo_gate(args: &[String]) -> Result<(), DynError> {
    let thresholds = SloThresholds::parse_args(args)?;
    let (runtime_status, runtime_violations, concurrent_status, concurrent_violations) =
        collect_health_snapshot(&thresholds)?;
    if runtime_status != "OK" || concurrent_status != "OK" {
        return Err(std::io::Error::other(format!(
            "SLO gate failed: runtime={} ({}) | concurrent={} ({})",
            runtime_status,
            runtime_violations.join("; "),
            concurrent_status,
            concurrent_violations.join("; ")
        ))
        .into());
    }

    tracing::info!(
        "SLO gate passed: profile={} runtime=OK concurrent=OK",
        thresholds.profile_name()
    );
    Ok(())
}

fn cmd_metrics() -> Result<(), DynError> {
    let metrics = collect_prometheus_metrics(&SloThresholds::default())?;
    tracing::info!("{metrics}");
    Ok(())
}

pub(crate) fn cmd_serve_metrics(args: &[String]) -> Result<(), DynError> {
    let (
        addr,
        once,
        cache_ttl_requests,
        threshold_args,
        slo_profile_path,
        admin_token,
        slo_audit_path,
        disable_auto_rollback,
        slo_audit_max_bytes,
    ) = parse_serve_metrics_options(args)?;
    let listener = TcpListener::bind(&addr)?;
    tracing::info!(
        "Metrics endpoint listening on http://{}/metrics and /healthz",
        addr
    );
    let mut thresholds = SloThresholds::parse_args(&threshold_args)?;
    if let Some(path) = slo_profile_path.as_deref()
        && let Some(profile) = load_persisted_profile(path)?
    {
        thresholds = SloThresholds::parse_args(&["--profile".to_string(), profile])?;
    }
    let mut cached_metrics: Option<(u64, String)> = None;
    let mut cached_health: Option<(u64, HealthSnapshot)> = None;
    let mut served_requests = 0_u64;
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                tracing::error!("metrics connection failed: {err}");
                continue;
            }
        };
        let mut request = [0_u8; 2048];
        let read_len = stream.read(&mut request).unwrap_or_default();
        let request_line = String::from_utf8_lossy(&request[..read_len]);
        let is_metrics = request_line.starts_with("GET /metrics ");
        let is_healthz = request_line.starts_with("GET /healthz ");
        let is_readyz = request_line.starts_with("GET /readyz ");
        let is_livez = request_line.starts_with("GET /livez ");
        let is_admin_audit = request_line.starts_with("GET /admin/slo-audit");
        let is_admin_slo = request_line.starts_with("POST /admin/slo-profile");
        let (status_line, body) = if is_metrics {
            match get_cached_metrics(
                &thresholds,
                cache_ttl_requests,
                served_requests,
                &mut cached_metrics,
            ) {
                Ok(metrics) => ("HTTP/1.1 200 OK", metrics),
                Err(err) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!("metrics generation failed: {err}"),
                ),
            }
        } else if is_healthz {
            match get_cached_health(
                &thresholds,
                cache_ttl_requests,
                served_requests,
                &mut cached_health,
            ) {
                Ok((
                    runtime_status,
                    runtime_violations,
                    concurrent_status,
                    concurrent_violations,
                )) => {
                    let status_line = if runtime_status == "OK" && concurrent_status == "OK" {
                        "HTTP/1.1 200 OK"
                    } else {
                        "HTTP/1.1 503 Service Unavailable"
                    };
                    match health_snapshot_json(
                        &runtime_status,
                        &runtime_violations,
                        &concurrent_status,
                        &concurrent_violations,
                    ) {
                        Ok(payload) => (status_line, payload),
                        Err(err) => (
                            "HTTP/1.1 500 Internal Server Error",
                            format!("health serialization failed: {err}"),
                        ),
                    }
                }
                Err(err) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!("health generation failed: {err}"),
                ),
            }
        } else if is_readyz {
            match get_cached_health(
                &thresholds,
                cache_ttl_requests,
                served_requests,
                &mut cached_health,
            ) {
                Ok((
                    runtime_status,
                    runtime_violations,
                    concurrent_status,
                    concurrent_violations,
                )) => {
                    if runtime_status == "OK"
                        && concurrent_status == "OK"
                        && runtime_violations.is_empty()
                        && concurrent_violations.is_empty()
                    {
                        ("HTTP/1.1 200 OK", "ready".to_string())
                    } else {
                        ("HTTP/1.1 503 Service Unavailable", "not ready".to_string())
                    }
                }
                Err(err) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!("readiness evaluation failed: {err}"),
                ),
            }
        } else if is_livez {
            ("HTTP/1.1 200 OK", "alive".to_string())
        } else if is_admin_audit {
            if !is_authorized_admin_request(&request_line, admin_token.as_deref()) {
                (
                    "HTTP/1.1 401 Unauthorized",
                    "missing or invalid X-Admin-Token".to_string(),
                )
            } else if let Some(path) = slo_audit_path.as_deref() {
                let limit = parse_admin_audit_limit_from_request_line(&request_line)
                    .unwrap_or(DEFAULT_ADMIN_AUDIT_LIMIT)
                    .min(MAX_ADMIN_AUDIT_LIMIT);
                match read_admin_audit_json(path, limit) {
                    Ok(payload) => ("HTTP/1.1 200 OK", payload),
                    Err(err) => (
                        "HTTP/1.1 500 Internal Server Error",
                        format!("audit read failed: {err}"),
                    ),
                }
            } else {
                (
                    "HTTP/1.1 400 Bad Request",
                    "slo audit is disabled (configure --slo-audit-path)".to_string(),
                )
            }
        } else if is_admin_slo {
            if !is_authorized_admin_request(&request_line, admin_token.as_deref()) {
                (
                    "HTTP/1.1 401 Unauthorized",
                    "missing or invalid X-Admin-Token".to_string(),
                )
            } else {
                match parse_admin_profile_from_request_line(&request_line) {
                    Some(profile) => {
                        let previous_profile = thresholds.profile_name().to_string();
                        match SloThresholds::parse_args(&[
                            "--profile".to_string(),
                            profile.to_string(),
                        ]) {
                            Ok(updated) => {
                                let switch_allowed = if disable_auto_rollback {
                                    true
                                } else {
                                    profile_switch_guard_passes(&updated)?
                                };
                                if !switch_allowed {
                                    if let Some(path) = slo_audit_path.as_deref() {
                                        append_slo_audit_line(
                                            path,
                                            served_requests + 1,
                                            &previous_profile,
                                            profile,
                                            "rejected",
                                            "candidate profile failed readiness gate",
                                            slo_audit_max_bytes,
                                        )?;
                                    }
                                    (
                                        "HTTP/1.1 409 Conflict",
                                        format!(
                                            "profile switch blocked by auto-rollback guard: {} -> {}",
                                            previous_profile, profile
                                        ),
                                    )
                                } else {
                                    thresholds = updated;
                                    cached_metrics = None;
                                    cached_health = None;
                                    if let Some(path) = slo_profile_path.as_deref() {
                                        persist_profile(path, thresholds.profile_name())?;
                                    }
                                    if let Some(path) = slo_audit_path.as_deref() {
                                        append_slo_audit_line(
                                            path,
                                            served_requests + 1,
                                            &previous_profile,
                                            thresholds.profile_name(),
                                            "applied",
                                            "profile switch accepted",
                                            slo_audit_max_bytes,
                                        )?;
                                    }
                                    (
                                        "HTTP/1.1 200 OK",
                                        format!("active profile set to {}", thresholds.profile_name()),
                                    )
                                }
                            }
                            Err(err) => (
                                "HTTP/1.1 400 Bad Request",
                                format!("invalid profile update: {err}"),
                            ),
                        }
                    }
                    None => (
                        "HTTP/1.1 400 Bad Request",
                        "missing profile query param (expected ?profile=strict|balanced|exploratory)"
                            .to_string(),
                    ),
                }
            }
        } else {
            (
                "HTTP/1.1 404 Not Found",
                "not found; use /metrics, /healthz, /readyz, /livez, GET /admin/slo-audit, or POST /admin/slo-profile?profile=..."
                    .to_string(),
            )
        };
        let content_type = if is_metrics {
            "text/plain; version=0.0.4"
        } else if is_healthz || is_admin_audit {
            "application/json"
        } else {
            "text/plain"
        };
        let response = format!(
            "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        if let Err(err) = stream.write_all(response.as_bytes()) {
            tracing::error!("metrics response write failed: {err}");
        }
        served_requests += 1;
        if once && served_requests >= 1 {
            break;
        }
    }
    Ok(())
}

fn collect_prometheus_metrics(thresholds: &SloThresholds) -> Result<String, DynError> {
    let (runtime_status, runtime_violations, concurrent_status, concurrent_violations) =
        collect_health_snapshot(thresholds)?;
    let (mut pipeline, runtime_report) = run_runtime_persistence_checks_with_report()?;
    run_training_and_cas_checks(&mut pipeline)?;
    let concurrent_report = run_concurrent_pipeline_checks_with_report()?;
    Ok(format!(
        "{}\n{}\nmoe_runtime_slo_status {}\nmoe_concurrent_slo_status {}\nmoe_slo_profile{{profile=\"{}\"}} 1\n",
        runtime_report.to_prometheus_text("moe_runtime"),
        concurrent_report.to_prometheus_text("moe_concurrent"),
        if runtime_status == "OK" && runtime_violations.is_empty() {
            1
        } else {
            0
        },
        if concurrent_status == "OK" && concurrent_violations.is_empty() {
            1
        } else {
            0
        },
        thresholds.profile_name(),
    ))
}

pub(crate) fn parse_serve_metrics_options(
    args: &[String],
) -> Result<ServeMetricsOptions, DynError> {
    let mut addr = "127.0.0.1:9464".to_string();
    let mut once = false;
    let mut cache_ttl_requests = 1_u64;
    let mut slo_profile_path: Option<String> = None;
    let mut admin_token: Option<String> = None;
    let mut slo_audit_path: Option<String> = None;
    let mut disable_auto_rollback = false;
    let mut slo_audit_max_bytes: Option<u64> = None;
    let mut threshold_args = Vec::new();
    let mut idx = 0_usize;
    while idx < args.len() {
        let arg = &args[idx];
        if arg == "--once" {
            once = true;
            idx += 1;
        } else if arg == "--cache-ttl-requests" {
            let raw = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other("missing value for --cache-ttl-requests"))?;
            let ttl_requests: u64 = raw.parse()?;
            cache_ttl_requests = ttl_requests.max(1);
            idx += 2;
        } else if arg == "--slo-profile-path" {
            let raw = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other("missing value for --slo-profile-path"))?;
            slo_profile_path = Some(raw.to_string());
            idx += 2;
        } else if arg == "--admin-token" {
            let raw = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other("missing value for --admin-token"))?;
            admin_token = Some(raw.to_string());
            idx += 2;
        } else if arg == "--slo-audit-path" {
            let raw = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other("missing value for --slo-audit-path"))?;
            slo_audit_path = Some(raw.to_string());
            idx += 2;
        } else if arg == "--slo-audit-max-bytes" {
            let raw = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other("missing value for --slo-audit-max-bytes"))?;
            let max_bytes: u64 = raw.parse()?;
            slo_audit_max_bytes = Some(max_bytes.max(1));
            idx += 2;
        } else if arg == "--disable-auto-rollback" {
            disable_auto_rollback = true;
            idx += 1;
        } else if arg.starts_with("--") {
            threshold_args.push(arg.to_string());
            let value = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other(format!("missing value for {arg}")))?;
            threshold_args.push(value.to_string());
            idx += 2;
        } else if addr == "127.0.0.1:9464" {
            addr = arg.to_string();
            idx += 1;
        } else {
            return Err(
                std::io::Error::other(format!("unexpected positional argument: {arg}")).into(),
            );
        }
    }
    Ok((
        addr,
        once,
        cache_ttl_requests,
        threshold_args,
        slo_profile_path,
        admin_token,
        slo_audit_path,
        disable_auto_rollback,
        slo_audit_max_bytes,
    ))
}

fn collect_health_snapshot(thresholds: &SloThresholds) -> Result<HealthSnapshot, DynError> {
    let (mut pipeline, runtime_report) = run_runtime_persistence_checks_with_report()?;
    run_training_and_cas_checks(&mut pipeline)?;
    let concurrent_report = run_concurrent_pipeline_checks_with_report()?;

    let runtime_violations = runtime_report.slo_violations(
        thresholds.runtime_min_successes,
        thresholds.runtime_max_rejections,
        thresholds.runtime_max_parse_failures,
    );
    let concurrent_violations = concurrent_report.slo_violations(
        thresholds.concurrent_max_contention_rate,
        thresholds.concurrent_max_timeout_rate,
        thresholds.concurrent_min_successes,
        thresholds.concurrent_max_rejections,
        thresholds.concurrent_max_parse_failures,
    );

    Ok((
        if runtime_violations.is_empty() {
            "OK".to_string()
        } else {
            "FAIL".to_string()
        },
        runtime_violations,
        if concurrent_violations.is_empty() {
            "OK".to_string()
        } else {
            "FAIL".to_string()
        },
        concurrent_violations,
    ))
}

fn health_snapshot_json(
    runtime_status: &str,
    runtime_violations: &[String],
    concurrent_status: &str,
    concurrent_violations: &[String],
) -> Result<String, DynError> {
    let mut payload: std::collections::BTreeMap<&str, String> = std::collections::BTreeMap::new();
    payload.insert("runtime_status", runtime_status.to_string());
    payload.insert("runtime_violations", runtime_violations.join(" | "));
    payload.insert("concurrent_status", concurrent_status.to_string());
    payload.insert("concurrent_violations", concurrent_violations.join(" | "));
    common_json::json::to_json_string_pretty(&payload)
        .map_err(|err| std::io::Error::other(format!("health serialization failed: {err}")).into())
}

fn get_cached_metrics(
    thresholds: &SloThresholds,
    ttl_requests: u64,
    request_index: u64,
    cache: &mut Option<(u64, String)>,
) -> Result<String, DynError> {
    if let Some((updated_on_request, value)) = cache.as_ref()
        && request_index.saturating_sub(*updated_on_request) <= ttl_requests
    {
        return Ok(value.clone());
    }
    let fresh = collect_prometheus_metrics(thresholds)?;
    *cache = Some((request_index, fresh.clone()));
    Ok(fresh)
}

fn get_cached_health(
    thresholds: &SloThresholds,
    ttl_requests: u64,
    request_index: u64,
    cache: &mut Option<(u64, HealthSnapshot)>,
) -> Result<HealthSnapshot, DynError> {
    if let Some((updated_on_request, value)) = cache.as_ref()
        && request_index.saturating_sub(*updated_on_request) <= ttl_requests
    {
        return Ok(value.clone());
    }
    let fresh = collect_health_snapshot(thresholds)?;
    *cache = Some((request_index, fresh.clone()));
    Ok(fresh)
}

pub(crate) fn parse_admin_profile_from_request_line(request_line: &str) -> Option<&str> {
    parse_query_param_from_request_line(request_line, "POST ", "profile")
}

pub(crate) fn parse_admin_audit_limit_from_request_line(request_line: &str) -> Option<usize> {
    parse_query_param_from_request_line(request_line, "GET ", "limit")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
}

pub(crate) fn is_authorized_admin_request(request_text: &str, admin_token: Option<&str>) -> bool {
    let token = match admin_token {
        Some(value) => value,
        None => return true,
    };
    request_text.lines().any(|line| {
        if let Some((header, value)) = line.split_once(':') {
            header.eq_ignore_ascii_case("X-Admin-Token") && value.trim() == token
        } else {
            false
        }
    })
}

fn profile_switch_guard_passes(thresholds: &SloThresholds) -> Result<bool, DynError> {
    let (runtime_status, runtime_violations, concurrent_status, concurrent_violations) =
        collect_health_snapshot(thresholds)?;
    Ok(runtime_status == "OK"
        && concurrent_status == "OK"
        && runtime_violations.is_empty()
        && concurrent_violations.is_empty())
}

fn append_slo_audit_line(
    path: &str,
    seq: u64,
    from_profile: &str,
    to_profile: &str,
    result: &str,
    reason: &str,
    audit_max_bytes: Option<u64>,
) -> Result<(), DynError> {
    let audit_guard = audit_io_lock()
        .lock()
        .map_err(|_| std::io::Error::other("audit lock poisoned"))?;
    let line = format_slo_audit_entry_json(seq, from_profile, to_profile, result, reason);
    if let Some(max_bytes) = audit_max_bytes {
        rotate_audit_file_if_needed(path, max_bytes, line.len() as u64 + 1)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;
    drop(audit_guard);
    Ok(())
}

pub(crate) fn read_admin_audit_json(path: &str, limit: usize) -> Result<String, DynError> {
    if limit == 0 {
        return Ok("[]".to_string());
    }
    let audit_guard = audit_io_lock()
        .lock()
        .map_err(|_| std::io::Error::other("audit lock poisoned"))?;
    let result = match File::open(path) {
        Ok(file) => {
            let mut tail: VecDeque<String> = VecDeque::with_capacity(limit.min(1024));
            for line in BufReader::new(file).lines() {
                let line = line?;
                if line.is_empty() {
                    continue;
                }
                if tail.len() == limit {
                    tail.pop_front();
                }
                tail.push_back(line);
            }
            let body = tail.into_iter().collect::<Vec<_>>().join(",");
            Ok(format!("[{body}]"))
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok("[]".to_string()),
        Err(err) => Err(err.into()),
    };
    drop(audit_guard);
    result
}

pub(crate) fn rotate_audit_file_if_needed(
    path: &str,
    max_bytes: u64,
    incoming_bytes: u64,
) -> Result<(), DynError> {
    let current_size = match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => 0,
        Err(err) => return Err(err.into()),
    };
    if current_size.saturating_add(incoming_bytes) <= max_bytes {
        return Ok(());
    }

    let rotated_path = format!("{path}.1");
    match fs::remove_file(&rotated_path) {
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }
    match fs::rename(path, rotated_path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub(crate) fn format_slo_audit_entry_json(
    seq: u64,
    from_profile: &str,
    to_profile: &str,
    result: &str,
    reason: &str,
) -> String {
    let sanitized_reason = reason.replace('\n', " ");
    let escaped_from = escape_json_string(from_profile);
    let escaped_to = escape_json_string(to_profile);
    let escaped_result = escape_json_string(result);
    let escaped_reason = escape_json_string(&sanitized_reason);
    format!(
        "{{\"seq\":{seq},\"from_profile\":\"{escaped_from}\",\"to_profile\":\"{escaped_to}\",\"result\":\"{escaped_result}\",\"reason\":\"{escaped_reason}\"}}"
    )
}

fn parse_query_param_from_request_line<'a>(
    request_line: &'a str,
    method_prefix: &str,
    key: &str,
) -> Option<&'a str> {
    let path = request_line
        .strip_prefix(method_prefix)?
        .split_whitespace()
        .next()?;
    let (_, query) = path.split_once('?')?;
    for pair in query.split('&') {
        let (param_key, value) = pair.split_once('=')?;
        if param_key == key {
            return Some(value);
        }
    }
    None
}

fn escape_json_string(raw: &str) -> String {
    raw.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn load_persisted_profile(path: &str) -> Result<Option<String>, DynError> {
    match fs::read_to_string(path) {
        Ok(content) => {
            let trimmed = content.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed.to_string()))
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}

fn persist_profile(path: &str, profile: &str) -> Result<(), DynError> {
    fs::write(path, profile.as_bytes())?;
    Ok(())
}
