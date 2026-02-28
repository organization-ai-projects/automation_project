use crate::model::entity_id::EntityId;
use crate::model::item::Item;
use crate::model::machine::{Machine, MachineKind};
use crate::model::world::World;
use crate::persistence::world_snapshot::WorldSnapshot;
use crate::replay::replay_engine::ReplayEngine;
use crate::sim::sim_engine::SimEngine;
use crate::replay::sim_event::SimEvent;
use crate::flow::flow_graph::FlowGraph;

fn build_source_conveyor_sink_world() -> World {
    let mut world = World::new();
    world.add_machine(Machine::new(
        EntityId::new(1),
        MachineKind::Source {
            output: Item::new("iron"),
            rate: 1,
        },
    ));
    world.add_machine(Machine::new(EntityId::new(2), MachineKind::Conveyor));
    world.add_machine(Machine::new(EntityId::new(3), MachineKind::Sink));
    world.add_edge(EntityId::new(1), EntityId::new(2));
    world.add_edge(EntityId::new(2), EntityId::new(3));
    world
}

/// Integration test: Source -> Conveyor -> Sink, run N ticks,
/// assert sink consumed items, and verify snapshot checksum.
#[test]
fn source_conveyor_sink_flow() {
    let world = build_source_conveyor_sink_world();
    let mut engine = SimEngine::new(world);
    engine.run(5).unwrap();

    let consumed: u64 = engine
        .event_log
        .iter()
        .filter_map(|e| {
            if let SimEvent::Consumed { entity, amount, .. } = e {
                if entity.value() == 3 {
                    Some(*amount)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();
    assert!(consumed > 0, "Sink should have consumed at least one item");

    let snapshot = WorldSnapshot::from_world(&engine.world);
    assert!(!snapshot.checksum().is_empty(), "Checksum must not be empty");
}

/// Determinism test: same world + same tick count => identical snapshot checksum.
#[test]
fn determinism_same_inputs_same_checksum() {
    let world = build_source_conveyor_sink_world();

    let mut engine_a = SimEngine::new(world.clone());
    engine_a.run(10).unwrap();
    let snap_a = WorldSnapshot::from_world(&engine_a.world);

    let mut engine_b = SimEngine::new(world.clone());
    engine_b.run(10).unwrap();
    let snap_b = WorldSnapshot::from_world(&engine_b.world);

    assert_eq!(
        snap_a.checksum(),
        snap_b.checksum(),
        "Determinism violated: same inputs must yield same checksum"
    );
}

/// Replay test: replay must yield identical checksum at tick N.
#[test]
fn replay_identical_checksum() {
    let world = build_source_conveyor_sink_world();
    let mut engine = SimEngine::new(world.clone());
    engine.run(8).unwrap();
    let original_snapshot = WorldSnapshot::from_world(&engine.world);

    let replay = ReplayEngine::new(world, engine.event_log.clone());
    let replayed_snapshot = replay.replay(8).unwrap();

    assert_eq!(
        original_snapshot.checksum(),
        replayed_snapshot.checksum(),
        "Replay must produce the same snapshot checksum"
    );
}

/// Flow graph consistency check.
#[test]
fn flow_graph_consistency() {
    let mut g = FlowGraph::new();
    g.add_node(EntityId::new(1));
    g.add_node(EntityId::new(2));
    g.add_node(EntityId::new(3));
    g.add_edge(EntityId::new(1), EntityId::new(2));
    g.add_edge(EntityId::new(2), EntityId::new(3));
    assert!(g.is_consistent());
}
