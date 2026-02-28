use auto_render::executor::Executor;
use auto_render::intent::{CinematographyPayload, Intent, IntentId, IntentPayload, IntentVersion};
use auto_render::plan::Capability;
use auto_render::planner::{CinematographyPlanner, PlannerInput, WorldSnapshot};
use auto_render::policy::{ApprovalRule, Budget, CapabilitySet, PolicyEngine, PolicySnapshot};
use auto_render::world::WorldState;
use std::collections::HashSet;

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

fn make_snapshot() -> PolicySnapshot {
    PolicySnapshot {
        snapshot_id: "snap-test".to_string(),
        created_at: "2026-01-01T00:00:00Z".to_string(),
        allowed_capabilities: CapabilitySet::new(all_capabilities()),
        budget: Budget::default(),
        rules: vec![ApprovalRule::AutoApprove],
    }
}

fn portrait_intent() -> Intent {
    Intent {
        intent_id: IntentId("intent-portrait-001".to_string()),
        intent_version: IntentVersion(1),
        created_at: "2026-01-01T00:00:00Z".to_string(),
        author: Some("test".to_string()),
        payload: IntentPayload::Cinematography(CinematographyPayload {
            subject_description: "human portrait".to_string(),
            shot_type: "close_up".to_string(),
            lighting_style: "soft_studio".to_string(),
            background: "neutral_gray".to_string(),
            fov_deg: Some(50.0),
            camera_distance: Some(2.0),
        }),
    }
}

#[test]
fn intent_to_plan_to_execute_deterministic() {
    let intent = portrait_intent();
    let snapshot = make_snapshot();
    let input = PlannerInput {
        intent,
        policy_snapshot: snapshot.clone(),
        world_snapshot: WorldSnapshot {
            entities_count: 0,
            camera_fov: 60.0,
            has_lighting: false,
        },
    };

    let planner = CinematographyPlanner::new();
    let candidates = planner.plan(&input).expect("plan");
    assert!(!candidates.is_empty());

    let policy_engine = PolicyEngine::new(snapshot.clone());
    let best = candidates.into_iter().next().expect("candidate");
    let plan = policy_engine
        .approve_plan_candidate(&best)
        .expect("approve");

    let executor1 = Executor::new(PolicyEngine::new(snapshot.clone()));
    let mut world1 = WorldState::new();
    let result1 = executor1.execute(&plan, &mut world1).expect("execute 1");

    let executor2 = Executor::new(PolicyEngine::new(snapshot));
    let mut world2 = WorldState::new();
    let result2 = executor2.execute(&plan, &mut world2).expect("execute 2");

    assert_eq!(
        result1.fingerprint, result2.fingerprint,
        "Execution must be deterministic"
    );
    assert_eq!(result1.actions_applied, result2.actions_applied);
}

#[test]
fn plan_ron_roundtrip() {
    let intent = portrait_intent();
    let snapshot = make_snapshot();
    let input = PlannerInput {
        intent,
        policy_snapshot: snapshot.clone(),
        world_snapshot: WorldSnapshot {
            entities_count: 0,
            camera_fov: 60.0,
            has_lighting: false,
        },
    };

    let planner = CinematographyPlanner::new();
    let candidates = planner.plan(&input).expect("plan");
    let policy_engine = PolicyEngine::new(snapshot);
    let best = candidates.into_iter().next().expect("candidate");
    let plan = policy_engine
        .approve_plan_candidate(&best)
        .expect("approve");

    let serialized = ron::to_string(&plan).expect("serialize");
    let deserialized: auto_render::plan::Plan = ron::from_str(&serialized).expect("deserialize");
    assert_eq!(plan.metadata.plan_id, deserialized.metadata.plan_id);
}
