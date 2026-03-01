#![allow(dead_code)]
use crate::config::sim_config::SimConfig;
use crate::determinism::seed::Seed;
use crate::economy::economy_engine::EconomyEngine;
use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::map::node_id::NodeId;
use crate::reputation::reputation_engine::ReputationEngine;
use crate::rides::ride::Ride;
use crate::rides::ride_engine::RideEngine;
use crate::rides::ride_id::RideId;
use crate::rides::ride_kind::RideKind;
use crate::routing::router::Router;
use crate::scenario::scenario::Scenario;
use crate::shops::shop_engine::ShopEngine;
use crate::shops::shop_id::ShopId;
use crate::sim::sim_state::SimState;
use crate::visitors::preference::Preference;
use crate::visitors::visitor::Visitor;
use crate::visitors::visitor::VisitorStatus;
use crate::visitors::visitor_id::VisitorId;
use std::collections::BTreeMap;

/// Orchestrates all simulation subsystems. All state mutations happen here only.
pub struct SimEngine {
    seed: Seed,
    entrance_node: NodeId,
    replay_path: Option<String>,
}

impl SimEngine {
    /// Create a new engine + initial state from a scenario.
    pub fn new(scenario: &Scenario, seed: u64, config: &SimConfig) -> (Self, SimState) {
        let seed_val = Seed::new(seed);
        let mut graph = crate::map::path_graph::PathGraph::new();

        for n in &scenario.nodes {
            graph.add_node(crate::map::path_node::PathNode::new(
                NodeId::new(n.id),
                n.name.clone(),
            ));
        }
        for e in &scenario.edges {
            graph.add_edge(crate::map::path_edge::PathEdge::new(
                NodeId::new(e.from),
                NodeId::new(e.to),
                e.cost,
            ));
        }

        let mut rides: BTreeMap<RideId, Ride> = BTreeMap::new();
        for r in &scenario.rides {
            rides.insert(
                RideId::new(r.id),
                Ride::new(
                    RideId::new(r.id),
                    r.kind,
                    NodeId::new(r.node),
                    r.capacity,
                    r.ticks_per_ride,
                    r.price,
                ),
            );
        }

        let mut shops: BTreeMap<ShopId, _> = BTreeMap::new();
        for s in &scenario.shops {
            shops.insert(
                ShopId::new(s.id),
                crate::shops::shop::Shop::new(
                    ShopId::new(s.id),
                    NodeId::new(s.node),
                    s.name.clone(),
                    s.price,
                ),
            );
        }

        let entrance = NodeId::new(scenario.entrance_node);
        let mut state = SimState::new(
            graph,
            rides,
            shops,
            scenario.initial_budget as i64,
            scenario.initial_reputation as i32,
            config.clone(),
        );

        // Spawn visitors deterministically from seed.
        for i in 0..scenario.visitor_count {
            let pref = Self::make_preference(seed_val, i as u64, &scenario.rides);
            let visitor = Visitor::new(VisitorId::new(i), entrance, pref);
            state.add_visitor(visitor);
        }

        let engine = SimEngine {
            seed: seed_val,
            entrance_node: entrance,
            replay_path: None,
        };
        (engine, state)
    }

    /// Advance the simulation by one tick. All mutations go through here.
    pub fn tick(&self, state: &mut SimState, event_log: &mut EventLog) {
        state.clock.advance();
        let tick = state.clock.current();

        // 1. Process visitor decisions (sorted by VisitorId for determinism).
        self.process_visitors(state, event_log, tick);

        // 2. Process ride engines (sorted by RideId for determinism).
        RideEngine::tick(state, event_log, tick);

        // 3. Process shop engines (sorted by ShopId for determinism).
        ShopEngine::tick(state, event_log, tick);

        // 4. Reconcile economy.
        EconomyEngine::tick(state, tick);

        // 5. Update reputation.
        ReputationEngine::tick(state, tick);
    }

    pub fn replay_path(&self) -> Option<&str> {
        self.replay_path.as_deref()
    }

    // ── visitor logic ────────────────────────────────────────────────────────

    fn process_visitors(
        &self,
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: crate::time::tick::Tick,
    ) {
        let visitor_ids: Vec<VisitorId> = {
            let mut ids: Vec<VisitorId> = state.visitors.keys().copied().collect();
            ids.sort(); // deterministic order
            ids
        };

        for vid in visitor_ids {
            let status = state.visitors[&vid].status.clone();
            match status {
                VisitorStatus::Idle => {
                    self.decide_next_action(state, event_log, tick, vid);
                }
                VisitorStatus::Walking { ref path, step } => {
                    self.advance_walk(state, event_log, tick, vid, path.clone(), step);
                }
                VisitorStatus::Queued(_) => {
                    self.process_queued(state, event_log, tick, vid);
                }
                VisitorStatus::Riding(_) => {
                    // Handled by RideEngine
                }
                VisitorStatus::Shopping { .. } => {
                    // Handled by ShopEngine
                }
                VisitorStatus::Left => {}
            }
        }
    }

    /// Decide what an idle visitor does next.
    fn decide_next_action(
        &self,
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: crate::time::tick::Tick,
        vid: VisitorId,
    ) {
        // Try to find the best available ride by preference score,
        // then shortest path cost, then ride_id (stable tie-break).
        let entrance = self.entrance_node;
        let visitor_node = state.visitors[&vid].current_node;
        let visitor_pref = state.visitors[&vid].preference.clone();

        // Collect candidate rides (available and reachable).
        let mut candidates: Vec<(i32, u32, RideId, Vec<NodeId>)> = Vec::new();
        let ride_ids: Vec<RideId> = {
            let mut ids: Vec<RideId> = state.rides.keys().copied().collect();
            ids.sort();
            ids
        };
        for rid in &ride_ids {
            let ride = &state.rides[rid];
            if !ride.is_available() {
                continue;
            }
            let score = visitor_pref.score(&ride.kind);
            if let Some(route) = Router::find_path(&state.graph, visitor_node, ride.node) {
                let cost = route.len() as u32;
                candidates.push((score, cost, *rid, route.steps));
            }
        }

        if !candidates.is_empty() {
            // Sort: highest score first, then lowest cost, then lowest ride_id (deterministic).
            candidates.sort_by(|a, b| {
                b.0.cmp(&a.0)
                    .then(a.1.cmp(&b.1))
                    .then(a.2.cmp(&b.2))
            });
            let (_, _, target_rid, path) = candidates.remove(0);

            if path.len() <= 1 {
                // Already at the ride — join queue directly.
                self.join_queue(state, event_log, tick, vid, target_rid);
            } else {
                let v = state.visitors.get_mut(&vid).unwrap();
                v.status = VisitorStatus::Walking {
                    path,
                    step: 1, // start at step 1 (step 0 = current node)
                };
            }
            return;
        }

        // No rides available — try a shop.
        let shop_ids: Vec<ShopId> = {
            let mut ids: Vec<ShopId> = state.shops.keys().copied().collect();
            ids.sort();
            ids
        };
        for sid in &shop_ids {
            let shop_node = state.shops[sid].node;
            if let Some(route) = Router::find_path(&state.graph, visitor_node, shop_node) {
                if route.len() <= 1 {
                    // At shop already.
                    let shop_ticks = state.config.shop_visit_ticks as u32;
                    let v = state.visitors.get_mut(&vid).unwrap();
                    v.status = VisitorStatus::Shopping {
                        shop: *sid,
                        ticks_remaining: shop_ticks,
                    };
                } else {
                    let v = state.visitors.get_mut(&vid).unwrap();
                    v.status = VisitorStatus::Walking {
                        path: route.steps,
                        step: 1,
                    };
                }
                return;
            }
        }

        // Nothing to do — wander back toward entrance or leave.
        let v = state.visitors.get_mut(&vid).unwrap();
        if v.rides_completed > 0 || v.current_node == entrance {
            // Leave
            v.status = VisitorStatus::Left;
            let mood = v.mood.value();
            event_log.push(SimEvent::VisitorLeft {
                tick,
                visitor_id: vid,
                mood,
            });
        }
    }

    fn advance_walk(
        &self,
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: crate::time::tick::Tick,
        vid: VisitorId,
        path: Vec<NodeId>,
        step: usize,
    ) {
        if step >= path.len() {
            // Done walking — go idle to re-decide.
            let v = state.visitors.get_mut(&vid).unwrap();
            v.status = VisitorStatus::Idle;
            return;
        }

        let next_node = path[step];
        let v = state.visitors.get_mut(&vid).unwrap();
        v.current_node = next_node;

        if step + 1 >= path.len() {
            // Arrived at destination.
            v.status = VisitorStatus::Idle;
        } else {
            v.status = VisitorStatus::Walking {
                path: path.clone(),
                step: step + 1,
            };
        }

        // Check if we're now at a ride node — join queue if destination.
        let current_node = state.visitors[&vid].current_node;
        if matches!(state.visitors[&vid].status, VisitorStatus::Idle) {
            // Check if any ride is at this node and visitor was heading there.
            let ride_ids: Vec<RideId> = {
                let mut ids: Vec<RideId> = state.rides.keys().copied().collect();
                ids.sort();
                ids
            };
            for rid in ride_ids {
                if state.rides[&rid].node == current_node && state.rides[&rid].is_available() {
                    self.join_queue(state, event_log, tick, vid, rid);
                    break;
                }
            }
        }
    }

    fn join_queue(
        &self,
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: crate::time::tick::Tick,
        vid: VisitorId,
        rid: RideId,
    ) {
        {
            let ride = state.rides.get_mut(&rid).unwrap();
            ride.queue.enqueue(vid);
        }
        let v = state.visitors.get_mut(&vid).unwrap();
        v.status = VisitorStatus::Queued(rid);
        v.patience = crate::visitors::patience::Patience::new(
            crate::visitors::patience::Patience::INITIAL,
        );
        event_log.push(SimEvent::VisitorJoinedQueue {
            tick,
            visitor_id: vid,
            ride_id: rid,
        });
    }

    fn process_queued(
        &self,
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: crate::time::tick::Tick,
        vid: VisitorId,
    ) {
        // Decay patience each tick while queued.
        let (patience_done, ride_id) = {
            let v = state.visitors.get_mut(&vid).unwrap();
            v.patience = v.patience.decay();
            let rid = if let VisitorStatus::Queued(r) = v.status {
                r
            } else {
                return;
            };
            (v.patience.is_exhausted(), rid)
        };

        if patience_done {
            // Visitor gives up — remove from queue and leave.
            state.rides.get_mut(&ride_id).unwrap().queue.remove(vid);
            let v = state.visitors.get_mut(&vid).unwrap();
            v.status = VisitorStatus::Left;
            v.mood = v.mood.adjust(-20);
            let mood = v.mood.value();
            event_log.push(SimEvent::VisitorLeft {
                tick,
                visitor_id: vid,
                mood,
            });
        }
    }

    // ── preference derivation ────────────────────────────────────────────────

    fn make_preference(
        seed: Seed,
        visitor_idx: u64,
        rides: &[crate::scenario::scenario::RideDef],
    ) -> Preference {
        // Deterministic preference: derive from seed+visitor_idx, no RNG.
        let h = seed.derive(visitor_idx);
        let kinds: Vec<RideKind> = rides
            .iter()
            .map(|r| r.kind)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        if kinds.is_empty() {
            return Preference::new(vec![RideKind::Coaster]);
        }
        // Rotate the kind list by a derived offset for each visitor.
        let offset = (h as usize) % kinds.len();
        let mut preferred = kinds[offset..].to_vec();
        preferred.extend_from_slice(&kinds[..offset]);
        Preference::new(preferred)
    }
}
