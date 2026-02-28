use super::CliError;
use crate::executor::Executor;
use crate::plan::{Capability, Plan};
use crate::policy::{ApprovalRule, Budget, CapabilitySet, PolicyEngine, PolicySnapshot};
use crate::world::WorldState;
use common_ron::read_ron;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct ReplayCommand {
    pub plan_path: PathBuf,
    pub print_fingerprint: bool,
}

impl ReplayCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let plan: Plan = read_ron(&self.plan_path)
            .map_err(|e| CliError::Parse(format!("Failed to parse plan: {e}")))?;

        let snapshot = PolicySnapshot {
            snapshot_id: plan.metadata.policy_snapshot_id.clone(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            allowed_capabilities: CapabilitySet::new(all_capabilities()),
            budget: Budget::default(),
            rules: vec![ApprovalRule::AutoApprove],
        };

        let policy_engine = PolicyEngine::new(snapshot);
        let executor = Executor::new(policy_engine);
        let mut world = WorldState::new();

        let result = executor
            .execute(&plan, &mut world)
            .map_err(crate::error::EngineError::Executor)?;

        println!(
            "Replay complete: {} actions applied in {}ms",
            result.actions_applied, result.elapsed_ms
        );

        if self.print_fingerprint {
            println!("World fingerprint: {}", result.fingerprint);
        }
        println!("Replay artifacts written to auto_render_output");

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
