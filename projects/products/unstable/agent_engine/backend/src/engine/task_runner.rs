//! projects/products/unstable/agent_engine/backend/src/engine/task_runner.rs
use std::fs;

use crate::{
    diagnostics::AgentEngineError,
    engine::{
        AgentOutcome, CopyInputStep, ExecutionContext, LogStep, SetOutputStep, Step, StepExecutor,
        StepSpec, Task,
    },
    protocol::cli_io,
};

pub fn run_cli(args: Vec<String>) -> Result<(), AgentEngineError> {
    if args.len() != 3 || args[1] != "run" {
        return Err(AgentEngineError::Usage(
            "usage: agent_engine_backend run <task.json>".to_string(),
        ));
    }

    let task_json = fs::read_to_string(&args[2])?;
    let task: Task = common_json::from_json_str(&task_json)?;
    let outcome = run_task(task)?;
    cli_io::write_outcome(&outcome)?;
    Ok(())
}

pub fn run_task(task: Task) -> Result<AgentOutcome, AgentEngineError> {
    let task_id = task.id.clone();
    let mut ctx = ExecutionContext::new(task.clone());
    let steps = materialize_steps(&task.steps);
    let step_results = StepExecutor::run(&mut ctx, &steps)?;

    Ok(AgentOutcome {
        task_id,
        success: true,
        step_results,
        output: ctx.output,
        logs: ctx.logs,
    })
}

fn materialize_steps(step_specs: &[StepSpec]) -> Vec<Box<dyn Step>> {
    let mut out: Vec<Box<dyn Step>> = Vec::new();
    for spec in step_specs {
        match spec {
            StepSpec::Log { message } => out.push(Box::new(LogStep {
                message: message.clone(),
            })),
            StepSpec::SetOutput { key, value } => out.push(Box::new(SetOutputStep {
                key: key.clone(),
                value: value.clone(),
            })),
            StepSpec::CopyInput {
                input_key,
                output_key,
            } => out.push(Box::new(CopyInputStep {
                input_key: input_key.clone(),
                output_key: output_key.clone(),
            })),
        }
    }
    out
}
