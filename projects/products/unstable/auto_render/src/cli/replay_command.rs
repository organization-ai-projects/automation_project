use super::CliError;
use crate::assets::FileAssetGenerator;
use crate::executor::Executor;
use crate::plan::{Capability, Plan};
use crate::policy::{ApprovalRule, Budget, CapabilitySet, PolicyEngine, PolicySnapshot};
use crate::renderer::FrameDumpRenderer;
use crate::world::WorldState;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct ReplayCommand {
    pub plan_path: PathBuf,
    pub print_fingerprint: bool,
}

impl ReplayCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let content = std::fs::read_to_string(&self.plan_path)?;
        let plan: Plan = ron::from_str(&content)
            .map_err(|e| CliError::Parse(format!("Failed to parse plan: {e}")))?;

        let snapshot = PolicySnapshot {
            snapshot_id: plan.metadata.policy_snapshot_id.clone(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            allowed_capabilities: CapabilitySet::new(all_capabilities()),
            budget: Budget::default(),
            rules: vec![ApprovalRule::AutoApprove],
        };

        let policy_engine = PolicyEngine::new(snapshot);
        let output_root = self
            .plan_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("replay_output");
        let asset_generator = Box::new(FileAssetGenerator::new(output_root.join("assets")));
        let renderer = Box::new(FrameDumpRenderer::new(output_root.join("frames")));
        let executor = Executor::with_backends(policy_engine, asset_generator, renderer);
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
        println!("Replay artifacts written to {}", output_root.display());

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
