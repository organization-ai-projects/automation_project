use super::CliError;
use crate::intent::Intent;
use crate::plan::Capability;
use crate::planner::{CinematographyPlanner, PlannerInput, WorldSnapshot};
use crate::policy::{ApprovalRule, Budget, CapabilitySet, PolicyEngine, PolicySnapshot};
use common_json::to_string_pretty;
use common_ron::{read_ron, write_ron};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct PlanCommand {
    pub intent_paths: Vec<PathBuf>,
    pub out_path: PathBuf,
    pub trace_path: Option<PathBuf>,
}

impl PlanCommand {
    pub fn run(&self) -> Result<(), CliError> {
        for intent_path in &self.intent_paths {
            let intent: Intent = read_ron(intent_path)
                .map_err(|e| CliError::Parse(format!("Failed to parse intent: {e}")))?;

            tracing::info!("Planning for intent: {:?}", intent.intent_id);

            let snapshot = PolicySnapshot {
                snapshot_id: "snap-default".to_string(),
                created_at: "2026-01-01T00:00:00Z".to_string(),
                allowed_capabilities: CapabilitySet::new(all_capabilities()),
                budget: Budget::default(),
                rules: vec![ApprovalRule::AutoApprove],
            };

            let planner_input = PlannerInput {
                intent,
                policy_snapshot: snapshot.clone(),
                world_snapshot: WorldSnapshot {
                    entities_count: 0,
                    camera_fov: 60.0,
                    has_lighting: false,
                },
            };

            let planner = CinematographyPlanner::new();
            let candidates = planner
                .plan(&planner_input)
                .map_err(crate::error::EngineError::Planner)?;

            let policy_engine = PolicyEngine::new(snapshot);
            let best = candidates
                .into_iter()
                .find(|c| c.constraints_violated.is_empty())
                .ok_or_else(|| CliError::Parse("No valid plan candidate found".to_string()))?;

            let plan = policy_engine
                .approve_plan_candidate(&best)
                .map_err(crate::error::EngineError::Policy)?;

            let out_path = if self.intent_paths.len() == 1 {
                self.out_path.clone()
            } else {
                let stem = intent_path.file_stem().unwrap_or_default();
                self.out_path
                    .with_file_name(format!("{}.plan.ron", stem.to_string_lossy()))
            };

            if let Some(parent) = out_path.parent().filter(|p| !p.as_os_str().is_empty()) {
                std::fs::create_dir_all(parent)?;
            }

            write_ron(&out_path, &plan)
                .map_err(|e| crate::error::EngineError::Serialization(e.to_string()))?;
            println!("Plan written to {}", out_path.display());

            if let Some(trace_path) = &self.trace_path {
                let trace = to_string_pretty(&best.explanation_trace)
                    .map_err(|e| crate::error::EngineError::Serialization(e.to_string()))?;
                if let Some(parent) = trace_path.parent().filter(|p| !p.as_os_str().is_empty()) {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(trace_path, trace)?;
                println!("Trace written to {}", trace_path.display());
            }
        }
        Ok(())
    }
}

fn all_capabilities() -> HashSet<Capability> {
    [
        Capability::WorldRead,
        Capability::WorldSpawnEntity,
        Capability::WorldSetTransform,
        Capability::WorldSetComponent,
        Capability::CameraSet,
        Capability::LightingSet,
        Capability::AssetSpecify,
        Capability::AssetGenerate,
        Capability::IoReadDisk,
        Capability::IoWriteDisk,
    ]
    .into_iter()
    .collect()
}
