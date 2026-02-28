// projects/products/unstable/autonomy_orchestrator_ai/src/planner_output.rs
use crate::artifacts::{PlannerOutputArtifact, ValidationInvocationArtifact};
use crate::domain::{CommandLineSpec, PlannerEdge, PlannerNode};
use common_json::{Json, JsonAccess, from_str};
use std::fs;

#[derive(Debug, Clone)]
pub struct PlannerOutput {
    pub execution_max_iterations: Option<u32>,
    pub reviewer_remediation_max_cycles: Option<u32>,
    pub remediation_steps: Vec<String>,
    pub validation_commands: Vec<ValidationInvocationArtifact>,
    pub memory_signal_codes: Vec<String>,
    pub planner_nodes: Vec<PlannerNode>,
    pub planner_edges: Vec<PlannerEdge>,
}

pub fn read_planner_output_from_artifacts(
    artifacts: &[String],
) -> Result<Option<PlannerOutputArtifact>, String> {
    for artifact in artifacts {
        if !artifact.ends_with(".json") {
            continue;
        }
        let raw = match fs::read_to_string(artifact) {
            Ok(raw) => raw,
            Err(_) => continue,
        };

        let parsed: Json = match from_str(&raw) {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };
        let payload = match parse_planner_output(&parsed) {
            Ok(Some(payload)) => payload,
            Ok(None) => continue,
            Err(err) => {
                return Err(format!(
                    "Invalid planner output artifact '{}': {}",
                    artifact, err
                ));
            }
        };
        return Ok(Some(PlannerOutputArtifact {
            source_path: artifact.clone(),
            payload,
        }));
    }
    Ok(None)
}

fn parse_planner_output(root: &Json) -> Result<Option<PlannerOutput>, String> {
    if let Ok(nested) = root.get_field("planner_output") {
        return parse_planner_payload(nested).map(Some);
    }
    parse_planner_payload(root).map(|payload| {
        if payload.execution_max_iterations.is_some()
            || payload.reviewer_remediation_max_cycles.is_some()
            || !payload.remediation_steps.is_empty()
            || !payload.validation_commands.is_empty()
            || !payload.memory_signal_codes.is_empty()
            || !payload.planner_nodes.is_empty()
        {
            Some(payload)
        } else {
            None
        }
    })
}

fn parse_planner_payload(payload: &Json) -> Result<PlannerOutput, String> {
    let execution_max_iterations = match payload.get_field("execution_max_iterations") {
        Ok(v) => Some({
            let raw = v
                .as_u64_strict()
                .map_err(|_| "execution_max_iterations must be an unsigned integer".to_string())?;
            u32::try_from(raw).map_err(|_| "execution_max_iterations is too large".to_string())
        }?),
        Err(_) => None,
    };

    let reviewer_remediation_max_cycles = match payload.get_field("reviewer_remediation_max_cycles")
    {
        Ok(v) => Some({
            let raw = v.as_u64_strict().map_err(|_| {
                "reviewer_remediation_max_cycles must be an unsigned integer".to_string()
            })?;
            u32::try_from(raw)
                .map_err(|_| "reviewer_remediation_max_cycles is too large".to_string())
        }?),
        Err(_) => None,
    };

    let remediation_steps = match payload.get_field("remediation_steps") {
        Ok(value) => value
            .as_array_strict()
            .map_err(|_| "remediation_steps must be an array".to_string())?
            .iter()
            .map(|entry| {
                entry
                    .as_str_strict()
                    .map(ToString::to_string)
                    .map_err(|_| "remediation_steps entries must be strings".to_string())
            })
            .collect::<Result<Vec<_>, String>>()?,
        Err(_) => Vec::new(),
    };

    let validation_commands = match payload.get_field("validation_commands") {
        Ok(value) => value
            .as_array_strict()
            .map_err(|_| "validation_commands must be an array".to_string())?
            .iter()
            .map(|entry| {
                let command = entry
                    .get_field("command")
                    .and_then(|v| v.as_str_strict())
                    .map(ToString::to_string)
                    .map_err(|_| "validation_commands[].command must be a string".to_string())?;
                let args = match entry.get_field("args") {
                    Ok(args) => args
                        .as_array_strict()
                        .map_err(|_| "validation_commands[].args must be an array".to_string())?
                        .iter()
                        .map(|arg| {
                            arg.as_str_strict().map(ToString::to_string).map_err(|_| {
                                "validation_commands[].args entries must be strings".to_string()
                            })
                        })
                        .collect::<Result<Vec<_>, String>>()?,
                    Err(_) => Vec::new(),
                };
                Ok(ValidationInvocationArtifact {
                    command_line: CommandLineSpec { command, args },
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        Err(_) => Vec::new(),
    };

    let planner_nodes = match payload.get_field("planner_nodes") {
        Ok(value) => value
            .as_array_strict()
            .map_err(|_| "planner_nodes must be an array".to_string())?
            .iter()
            .map(|entry| {
                let id = entry
                    .get_field("id")
                    .and_then(|v| v.as_str_strict())
                    .map(ToString::to_string)
                    .map_err(|_| "planner_nodes[].id must be a string".to_string())?;
                let action = entry
                    .get_field("action")
                    .and_then(|v| v.as_str_strict())
                    .map(ToString::to_string)
                    .map_err(|_| "planner_nodes[].action must be a string".to_string())?;
                Ok(PlannerNode { id, action })
            })
            .collect::<Result<Vec<_>, String>>()?,
        Err(_) => Vec::new(),
    };

    let planner_edges = match payload.get_field("planner_edges") {
        Ok(value) => value
            .as_array_strict()
            .map_err(|_| "planner_edges must be an array".to_string())?
            .iter()
            .map(|entry| {
                let from = entry
                    .get_field("from")
                    .and_then(|v| v.as_str_strict())
                    .map(ToString::to_string)
                    .map_err(|_| "planner_edges[].from must be a string".to_string())?;
                let to = entry
                    .get_field("to")
                    .and_then(|v| v.as_str_strict())
                    .map(ToString::to_string)
                    .map_err(|_| "planner_edges[].to must be a string".to_string())?;
                let condition_code = entry
                    .get_field("condition_code")
                    .and_then(|v| v.as_str_strict())
                    .map(ToString::to_string)
                    .unwrap_or_default();
                Ok(PlannerEdge {
                    from,
                    to,
                    condition_code,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        Err(_) => Vec::new(),
    };

    Ok(PlannerOutput {
        execution_max_iterations,
        reviewer_remediation_max_cycles,
        remediation_steps,
        validation_commands,
        memory_signal_codes: match payload.get_field("memory_signal_codes") {
            Ok(value) => value
                .as_array_strict()
                .map_err(|_| "memory_signal_codes must be an array".to_string())?
                .iter()
                .map(|entry| {
                    entry
                        .as_str_strict()
                        .map(ToString::to_string)
                        .map_err(|_| "memory_signal_codes entries must be strings".to_string())
                })
                .collect::<Result<Vec<_>, String>>()?,
            Err(_) => Vec::new(),
        },
        planner_nodes,
        planner_edges,
    })
}
