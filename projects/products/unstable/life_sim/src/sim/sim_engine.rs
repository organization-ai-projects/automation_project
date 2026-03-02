use crate::actions::{Action, ActionCost, ActionEffect, ActionKind};
use crate::config::SimConfig;
use crate::decision::{DecisionContext, DecisionEngine};
use crate::diagnostics::LifeSimError;
use crate::model::agent_id::AgentId;
use crate::model::{Agent, World};
use crate::needs::NeedKind;
use crate::report::{AgentReport, RunHash, RunReport, WorldSnapshot};
use crate::schedule::ScheduleEngine;
use crate::sim::sim_event::SimEvent;
use crate::sim::sim_state::SimState;
use crate::time::Tick;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub struct SimEngine {
    pub config: SimConfig,
}

impl SimEngine {
    pub fn new(config: SimConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, state: &mut SimState) -> Result<RunReport, LifeSimError> {
        // RNG seeded from state for future randomized logic (e.g., random events, action outcomes)
        let _rng = SmallRng::seed_from_u64(state.seed);

        // Track actions taken per agent
        let mut actions_taken: BTreeMap<AgentId, u64> =
            state.world.agents.keys().map(|id| (*id, 0u64)).collect();

        state.event_log.push(SimEvent::SimulationStarted {
            tick: state.clock.current(),
        });

        for _ in 0..self.config.ticks {
            state.clock.advance();
            let tick = state.clock.current();

            // Collect agent ids in sorted order for determinism
            let agent_ids: Vec<AgentId> = state.world.agents.keys().cloned().collect();

            // Decay needs for all agents
            for agent_id in &agent_ids {
                if let Some(agent) = state.world.agents.get_mut(agent_id) {
                    let old_needs = agent.needs.values.clone();
                    agent.needs.decay_tick();
                    for (kind, old_val) in &old_needs {
                        let new_val = agent.needs.get(*kind);
                        if new_val.0 != old_val.0 {
                            state.event_log.push(SimEvent::NeedChanged {
                                tick,
                                agent_id: *agent_id,
                                need_kind: *kind,
                                old_val: old_val.0,
                                new_val: new_val.0,
                            });
                        }
                    }
                }
            }

            // Decision engine per agent
            for agent_id in &agent_ids {
                let agent = match state.world.agents.get(agent_id) {
                    Some(a) => a.clone(),
                    None => continue,
                };

                let available_actions = build_available_actions(&agent, &state.world, tick);

                let ctx = DecisionContext {
                    agent_id: *agent_id,
                    tick,
                    available_actions,
                };

                if let Some(action) = DecisionEngine::pick(&agent, &ctx) {
                    // Apply action effects
                    if let Some(agent_mut) = state.world.agents.get_mut(agent_id) {
                        for (kind, delta) in &action.effect.need_deltas {
                            let old = agent_mut.needs.get(*kind);
                            let new_val = old.saturating_add_i32(*delta);
                            agent_mut.needs.set(*kind, new_val);
                            state.event_log.push(SimEvent::NeedChanged {
                                tick,
                                agent_id: *agent_id,
                                need_kind: *kind,
                                old_val: old.0,
                                new_val: new_val.0,
                            });
                        }
                    }
                    *actions_taken.entry(*agent_id).or_insert(0) += 1;
                    state.event_log.push(SimEvent::AgentActed {
                        tick,
                        agent_id: *agent_id,
                        action: action.kind,
                    });
                }
            }
        }

        state.event_log.push(SimEvent::SimulationEnded {
            tick: state.clock.current(),
        });

        let report = self.build_report(state, &actions_taken);
        Ok(report)
    }

    fn build_report(&self, state: &SimState, actions_taken: &BTreeMap<AgentId, u64>) -> RunReport {
        let agents: Vec<AgentReport> = state
            .world
            .agents
            .values()
            .map(|a| AgentReport {
                agent_id: a.id,
                name: a.name.clone(),
                final_needs: a.needs.clone(),
                memory_count: a.memory.len(),
                actions_taken: *actions_taken.get(&a.id).unwrap_or(&0),
            })
            .collect();

        let snapshot = WorldSnapshot {
            tick: state.clock.current(),
            agent_count: state.world.agents.len(),
            room_count: state.world.rooms.len(),
            object_count: state.world.objects.len(),
        };

        let run_hash = compute_run_hash(state.seed, self.config.ticks, &agents, &snapshot);

        RunReport {
            seed: state.seed,
            ticks_simulated: self.config.ticks,
            agents,
            snapshot,
            run_hash,
        }
    }
}

fn build_available_actions(agent: &Agent, world: &World, tick: Tick) -> Vec<Action> {
    let mut actions = Vec::new();

    // Check schedule first
    if let Some(scheduled) = ScheduleEngine::scheduled_action(tick, &agent.schedule) {
        actions.push(make_action(scheduled, None, None));
        return actions;
    }

    // Always available actions
    actions.push(make_action(ActionKind::Eat, None, None));
    actions.push(make_action(ActionKind::Sleep, None, None));
    actions.push(make_action(ActionKind::Relax, None, None));
    actions.push(make_action(ActionKind::UseBathroom, None, None));
    actions.push(make_action(ActionKind::Work, None, None));

    // Social actions towards other agents in same room
    for (other_id, other_agent) in &world.agents {
        if *other_id != agent.id && other_agent.room == agent.room {
            actions.push(make_action(ActionKind::SocialChat, Some(*other_id), None));
        }
    }

    // Object interactions for objects in same room
    for (obj_id, obj) in &world.objects {
        if obj.room == agent.room {
            actions.push(make_action(ActionKind::UseObject, None, Some(*obj_id)));
        }
    }

    actions
}

fn make_action(
    kind: ActionKind,
    target_agent: Option<crate::model::AgentId>,
    target_object: Option<crate::model::ObjectId>,
) -> Action {
    let (need_deltas, ticks) = match kind {
        ActionKind::Eat => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Hunger, 20i32);
            (d, 2)
        }
        ActionKind::Sleep => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Energy, 30i32);
            (d, 8)
        }
        ActionKind::SocialChat => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Social, 15i32);
            d.insert(NeedKind::Fun, 5i32);
            (d, 2)
        }
        ActionKind::UseObject => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Fun, 10i32);
            (d, 1)
        }
        ActionKind::Work => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Energy, -5i32);
            (d, 4)
        }
        ActionKind::Relax => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Comfort, 10i32);
            d.insert(NeedKind::Fun, 5i32);
            (d, 2)
        }
        ActionKind::UseBathroom => {
            let mut d = BTreeMap::new();
            d.insert(NeedKind::Bladder, 30i32);
            d.insert(NeedKind::Hygiene, 5i32);
            (d, 1)
        }
        ActionKind::Move => (BTreeMap::new(), 1),
    };

    Action {
        kind,
        target_agent,
        target_object,
        cost: ActionCost { ticks },
        effect: ActionEffect { need_deltas },
    }
}

fn compute_run_hash(
    seed: u64,
    ticks_simulated: u64,
    agents: &[AgentReport],
    snapshot: &WorldSnapshot,
) -> RunHash {
    // Serialize without run_hash field
    #[derive(serde::Serialize)]
    struct HashableReport<'a> {
        seed: u64,
        ticks_simulated: u64,
        agents: &'a [AgentReport],
        snapshot: &'a WorldSnapshot,
    }

    let hashable = HashableReport {
        seed,
        ticks_simulated,
        agents,
        snapshot,
    };

    let canonical = serde_json::to_string(&hashable).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let result = hasher.finalize();
    RunHash(hex::encode(result))
}
