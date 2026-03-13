//! projects/products/unstable/neurosymbolic_moe/backend/src/app.rs
use std::path::PathBuf;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::aggregator::AggregationStrategy;
use crate::apps::{
    DynError, cmd_impl_check, run_concurrent_pipeline_checks_with_report,
    run_runtime_persistence_checks_with_report, run_training_and_cas_checks,
};
use crate::echo_expert::EchoExpert;
use crate::moe_core::{self, ExpertCapability, Task, TaskPriority, TaskType};
use crate::orchestrator::{MoePipeline, MoePipelineBuilder};
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
        "impl-check" => cmd_impl_check(),
        "slo-gate" => cmd_slo_gate(),
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
    let input = cli_input_or_default(args);
    let mut pipeline = build_cli_pipeline();
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

fn cli_input_or_default(args: &[String]) -> String {
    if args.is_empty() {
        "default task input".to_string()
    } else {
        args.join(" ")
    }
}

fn build_cli_pipeline() -> MoePipeline {
    MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(3)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
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
}

fn print_usage() {
    tracing::info!("neurosymbolic_moe - advanced modular MoE platform");
    tracing::info!("");
    tracing::info!("Commands:");
    tracing::info!("  run [input...]     Execute a task through the MoE pipeline");
    tracing::info!("  status             Show platform component status");
    tracing::info!("  trace [path]       Inspect execution traces");
    tracing::info!("  impl-check         Execute full component wiring check");
    tracing::info!("  slo-gate           Fail-fast SLO gate for CI");
    tracing::info!("  metrics            Print Prometheus-compatible metrics snapshot");
    tracing::info!("  serve-metrics [addr]  Serve /metrics over HTTP (default 127.0.0.1:9464)");
}

fn cmd_slo_gate() -> Result<(), DynError> {
    let (mut pipeline, runtime_report) = run_runtime_persistence_checks_with_report()?;
    run_training_and_cas_checks(&mut pipeline)?;
    let concurrent_report = run_concurrent_pipeline_checks_with_report()?;

    let runtime_status = runtime_report.slo_status(1, 0, 0);
    let concurrent_status = concurrent_report.slo_status(1.0, 0.2, 1, 0, 0);
    if runtime_status != "OK" || concurrent_status != "OK" {
        let mut messages = Vec::new();
        if runtime_status != "OK" {
            messages.push(format!(
                "runtime SLO failed: {}",
                runtime_report.slo_violations(1, 0, 0).join("; ")
            ));
        }
        if concurrent_status != "OK" {
            messages.push(format!(
                "concurrent SLO failed: {}",
                concurrent_report
                    .slo_violations(1.0, 0.2, 1, 0, 0)
                    .join("; ")
            ));
        }
        return Err(std::io::Error::other(messages.join(" | ")).into());
    }

    tracing::info!("SLO gate passed: runtime=OK concurrent=OK");
    Ok(())
}

fn cmd_metrics() -> Result<(), DynError> {
    let metrics = collect_prometheus_metrics()?;
    tracing::info!("{metrics}");
    Ok(())
}

fn cmd_serve_metrics(args: &[String]) -> Result<(), DynError> {
    let addr = args
        .first()
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:9464".to_string());
    let listener = TcpListener::bind(&addr)?;
    tracing::info!("Metrics endpoint listening on http://{addr}/metrics");
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
        let (status_line, body) = if is_metrics {
            match collect_prometheus_metrics() {
                Ok(metrics) => ("HTTP/1.1 200 OK", metrics),
                Err(err) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!("metrics generation failed: {err}"),
                ),
            }
        } else {
            (
                "HTTP/1.1 404 Not Found",
                "not found; use /metrics".to_string(),
            )
        };
        let content_type = if is_metrics {
            "text/plain; version=0.0.4"
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
    }
    Ok(())
}

fn collect_prometheus_metrics() -> Result<String, DynError> {
    let (mut pipeline, runtime_report) = run_runtime_persistence_checks_with_report()?;
    run_training_and_cas_checks(&mut pipeline)?;
    let concurrent_report = run_concurrent_pipeline_checks_with_report()?;
    Ok(format!(
        "{}\n{}",
        runtime_report.to_prometheus_text("moe_runtime"),
        concurrent_report.to_prometheus_text("moe_concurrent")
    ))
}
