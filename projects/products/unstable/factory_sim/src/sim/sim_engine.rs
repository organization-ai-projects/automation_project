use crate::diagnostics::error::SimError;
use crate::model::entity_id::EntityId;
use crate::model::item::Item;
use crate::model::machine::MachineKind;
use crate::model::world::World;
use crate::replay::sim_event::SimEvent;

/// Drives the simulation forward one tick at a time.
/// Iteration order over entities is deterministic (sorted by EntityId).
pub struct SimEngine {
    pub world: World,
    pub event_log: Vec<SimEvent>,
}

impl SimEngine {
    pub fn new(world: World) -> Self {
        Self {
            world,
            event_log: Vec::new(),
        }
    }

    /// Advance the simulation by one tick.
    /// All changes are recorded as `SimEvent`s.
    pub fn step(&mut self) -> Result<(), SimError> {
        let tick = self.world.tick;
        let ids: Vec<EntityId> = self.world.machine_ids();

        // Phase 1: collect production and transfers (no mutation yet).
        let mut transfers: Vec<(EntityId, EntityId, String, u64)> = Vec::new();
        let mut productions: Vec<(EntityId, String, u64)> = Vec::new();

        for &id in &ids {
            let machine = self.world.get_machine(id).ok_or(SimError::UnknownEntity(id))?;
            match machine.kind.clone() {
                MachineKind::Source { output, rate } => {
                    productions.push((id, output.name.clone(), rate));
                    // Source also forwards newly produced items to downstream neighbors.
                    let neighbors = self.world.neighbors(id);
                    for n in &neighbors {
                        transfers.push((id, *n, output.name.clone(), rate));
                    }
                }
                MachineKind::Conveyor => {
                    // Move everything in inventory to each downstream neighbour (split evenly).
                    let neighbors = self.world.neighbors(id);
                    if !neighbors.is_empty() {
                        let machine = self.world.get_machine(id).ok_or(SimError::UnknownEntity(id))?;
                        for (item_name, &count) in machine.inventory.counts() {
                            if count > 0 {
                                let per_neighbor = count / neighbors.len() as u64;
                                if per_neighbor > 0 {
                                    for n in &neighbors {
                                        transfers.push((id, *n, item_name.clone(), per_neighbor));
                                    }
                                }
                            }
                        }
                    }
                }
                MachineKind::Sink | MachineKind::Transformer { .. } => {}
            }
        }

        // Phase 2: transformer recipes.
        let mut transformer_outputs: Vec<(EntityId, String, u64)> = Vec::new();
        let mut transformer_consumed: Vec<(EntityId, String, u64)> = Vec::new();
        for &id in &ids {
            let machine = self.world.get_machine(id).ok_or(SimError::UnknownEntity(id))?;
            if let MachineKind::Transformer { input, input_count, output, output_count } = machine.kind.clone() {
                let available = machine.inventory.count(&input);
                let batches = available / input_count;
                if batches > 0 {
                    transformer_consumed.push((id, input.name.clone(), batches * input_count));
                    transformer_outputs.push((id, output.name.clone(), batches * output_count));
                    // Transformer sends outputs downstream.
                    let neighbors = self.world.neighbors(id);
                    for n in &neighbors {
                        transfers.push((id, *n, output.name.clone(), batches * output_count));
                    }
                }
            }
        }

        // Phase 3: apply productions.
        for (id, item_name, amount) in &productions {
            let item = Item::new(item_name.as_str());
            let machine = self.world.get_machine_mut(*id).ok_or(SimError::UnknownEntity(*id))?;
            machine.inventory.add(&item, *amount);
            self.event_log.push(SimEvent::Produced {
                tick,
                entity: *id,
                item: item_name.clone(),
                amount: *amount,
            });
        }

        // Phase 4: apply conveyor and transformer transfers (remove from source, add to dest).
        for (from, to, item_name, amount) in &transfers {
            let item = Item::new(item_name.as_str());
            {
                let src = self.world.get_machine_mut(*from).ok_or(SimError::UnknownEntity(*from))?;
                src.inventory.remove(&item, *amount);
            }
            {
                let dst = self.world.get_machine_mut(*to).ok_or(SimError::UnknownEntity(*to))?;
                dst.inventory.add(&item, *amount);
            }
            self.event_log.push(SimEvent::Transferred {
                tick,
                from: *from,
                to: *to,
                item: item_name.clone(),
                amount: *amount,
            });
        }

        // Phase 5: apply transformer consumption.
        for (id, item_name, amount) in &transformer_consumed {
            let item = Item::new(item_name.as_str());
            let machine = self.world.get_machine_mut(*id).ok_or(SimError::UnknownEntity(*id))?;
            machine.inventory.remove(&item, *amount);
        }
        for (id, item_name, amount) in &transformer_outputs {
            self.event_log.push(SimEvent::Produced {
                tick,
                entity: *id,
                item: item_name.clone(),
                amount: *amount,
            });
        }

        // Phase 6: sinks consume their entire inventory.
        for &id in &ids {
            let machine = self.world.get_machine(id).ok_or(SimError::UnknownEntity(id))?;
            if machine.kind == MachineKind::Sink {
                let totals: Vec<(String, u64)> = machine
                    .inventory
                    .counts()
                    .iter()
                    .map(|(k, v)| (k.clone(), *v))
                    .collect();
                for (item_name, amount) in totals {
                    if amount > 0 {
                        let item = Item::new(item_name.as_str());
                        let machine = self.world.get_machine_mut(id).ok_or(SimError::UnknownEntity(id))?;
                        machine.inventory.remove(&item, amount);
                        self.event_log.push(SimEvent::Consumed {
                            tick,
                            entity: id,
                            item: item_name,
                            amount,
                        });
                    }
                }
            }
        }

        self.world.tick += 1;
        Ok(())
    }

    /// Run for `n` ticks.
    pub fn run(&mut self, n: u64) -> Result<(), SimError> {
        for _ in 0..n {
            self.step()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::machine::{Machine, MachineKind};
    use crate::model::item::Item;

    fn build_source_sink_world() -> World {
        let mut world = World::new();
        let src = Machine::new(EntityId::new(1), MachineKind::Source { output: Item::new("iron"), rate: 1 });
        let sink = Machine::new(EntityId::new(3), MachineKind::Sink);
        let conv = Machine::new(EntityId::new(2), MachineKind::Conveyor);
        world.add_machine(src);
        world.add_machine(conv);
        world.add_machine(sink);
        world.add_edge(EntityId::new(1), EntityId::new(2));
        world.add_edge(EntityId::new(2), EntityId::new(3));
        world
    }

    #[test]
    fn single_step_produces_item() {
        let mut engine = SimEngine::new(build_source_sink_world());
        engine.step().unwrap();
        // After 1 tick: source produces to conveyor inventory.
        let conv = engine.world.get_machine(EntityId::new(2)).unwrap();
        assert_eq!(conv.inventory.count(&Item::new("iron")), 1);
    }

    #[test]
    fn two_steps_flows_to_sink() {
        let mut engine = SimEngine::new(build_source_sink_world());
        engine.run(2).unwrap();
        // After tick 0: source->conv. After tick 1: conv->sink (consumed), source->conv again.
        let sink_events: u64 = engine.event_log.iter()
            .filter(|e| matches!(e, SimEvent::Consumed { entity, .. } if entity.value() == 3))
            .map(|e| if let SimEvent::Consumed { amount, .. } = e { *amount } else { 0 })
            .sum();
        assert!(sink_events > 0);
    }
}
