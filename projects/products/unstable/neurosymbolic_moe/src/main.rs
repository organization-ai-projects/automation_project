use std::path::PathBuf;

use neurosymbolic_moe::aggregator::AggregationStrategy;
use neurosymbolic_moe::moe_core::{
    ExecutionContext, ExpertCapability, ExpertId, ExpertMetadata, ExpertOutput, ExpertStatus,
    ExpertType, Task, TaskType,
};
use neurosymbolic_moe::orchestrator::MoePipelineBuilder;
use neurosymbolic_moe::policy_guard::{Policy, PolicyType};
use neurosymbolic_moe::router::HeuristicRouter;

struct EchoExpert {
    metadata: ExpertMetadata,
}

impl EchoExpert {
    fn new(id: &str, name: &str, capabilities: Vec<ExpertCapability>) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::new(id.to_string()),
                name: name.to_string(),
                version: "0.1.0".to_string(),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

impl neurosymbolic_moe::moe_core::Expert for EchoExpert {
    fn id(&self) -> &ExpertId {
        &self.metadata.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.metadata
    }

    fn can_handle(&self, _task: &Task) -> bool {
        true
    }

    fn execute(
        &self,
        task: &Task,
        _context: &ExecutionContext,
    ) -> Result<ExpertOutput, neurosymbolic_moe::moe_core::ExpertError> {
        Ok(ExpertOutput {
            expert_id: self.metadata.id.clone(),
            content: format!("[{}] processed: {}", self.metadata.name, task.input()),
            confidence: 0.9,
            metadata: std::collections::HashMap::new(),
            trace: vec![format!("Expert {} executed", self.metadata.name)],
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            Ok(())
        }
    }
}

fn cmd_run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let input = if args.is_empty() {
        "default task input".to_string()
    } else {
        args.join(" ")
    };

    let task_type = TaskType::CodeGeneration;

    let router = HeuristicRouter::new(3);
    let mut pipeline = MoePipelineBuilder::new()
        .with_router(Box::new(router))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_max_traces(1000)
        .build();

    pipeline.register_expert(Box::new(EchoExpert::new(
        "code_gen",
        "CodeGenerationExpert",
        vec![ExpertCapability::CodeGeneration],
    )))?;

    pipeline.register_expert(Box::new(EchoExpert::new(
        "code_transform",
        "CodeTransformExpert",
        vec![ExpertCapability::CodeTransformation],
    )))?;

    pipeline.register_expert(Box::new(EchoExpert::new(
        "validator",
        "ValidationExpert",
        vec![ExpertCapability::Validation],
    )))?;

    pipeline.add_policy(Policy {
        id: "length_check".to_string(),
        name: "Output Length Check".to_string(),
        description: "Ensures output is not too long".to_string(),
        policy_type: PolicyType::LengthLimit(10000),
        active: true,
    });

    let task = Task::new("task-001", task_type, input);

    match pipeline.execute(task) {
        Ok(result) => {
            println!("Pipeline execution successful");
            if let Some(selected) = &result.selected_output {
                println!("Selected expert: {}", selected.expert_id.as_str());
                println!("Confidence: {:.2}", selected.confidence);
                println!("Output: {}", selected.content);
            }
            println!("Total outputs: {}", result.outputs.len());
            println!("Strategy: {}", result.strategy);
        }
        Err(e) => {
            eprintln!("Pipeline execution failed: {e}");
        }
    }

    println!(
        "\nExpert registry: {} experts registered",
        pipeline.registry().count()
    );
    println!("Trace log: {} entries", pipeline.trace_logger().count());
    println!(
        "Dataset store: {} entries",
        pipeline.dataset_store().count()
    );

    Ok(())
}

fn cmd_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("neurosymbolic_moe platform v0.1.0");
    println!();
    println!("Components:");
    println!("  moe_core          - Expert trait, Task model, ExecutionContext");
    println!("  expert_registry   - Pluggable expert registration");
    println!("  router            - Heuristic task routing");
    println!("  retrieval_engine  - RAG retrieval abstraction");
    println!("  memory_engine     - Short-term and long-term memory");
    println!("  buffer_manager    - Working and session buffers");
    println!("  dataset_engine    - Incremental trace-to-dataset pipeline");
    println!("  evaluation_engine - Expert and routing metrics");
    println!("  feedback_engine   - Execution feedback and corrections");
    println!("  aggregator        - Multi-expert output aggregation");
    println!("  policy_guard      - Output validation and policy checks");
    println!("  trace_logger      - Execution traces and telemetry");
    println!("  orchestrator      - Main orchestration pipeline");
    Ok(())
}

fn cmd_trace(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let trace_path = if args.is_empty() {
        None
    } else {
        Some(PathBuf::from(&args[0]))
    };

    if let Some(path) = trace_path {
        println!("Trace output path: {}", path.display());
    }

    println!("Trace inspection not yet available in V0");
    Ok(())
}

fn print_usage() {
    println!("neurosymbolic_moe - advanced modular MoE platform");
    println!();
    println!("Commands:");
    println!("  run [input...]     Execute a task through the MoE pipeline");
    println!("  status             Show platform component status");
    println!("  trace [path]       Inspect execution traces");
}
