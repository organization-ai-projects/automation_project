use super::adjudication_report::AdjudicationReport;
use super::resolution_step::{ResolutionOutcome, ResolutionStep};
use crate::map::territory_id::TerritoryId;
use crate::model::game_state::GameState;
use crate::model::unit_id::UnitId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;

pub struct AdjudicationEngine {
    pub state: GameState,
}

impl AdjudicationEngine {
    pub fn new(state: GameState) -> Self {
        Self { state }
    }

    pub fn current_state(&self) -> &GameState {
        &self.state
    }

    /// Look up support count for (unit_id, target) in a sorted Vec.
    fn lookup_support(
        support_list: &[((UnitId, TerritoryId), u32)],
        key: (UnitId, TerritoryId),
    ) -> u32 {
        match support_list.binary_search_by_key(&key, |e| e.0) {
            Ok(i) => support_list[i].1,
            Err(_) => 0,
        }
    }

    /// Adjudicate a turn given multiple order sets (one per faction).
    /// Updates internal state and returns the adjudication report.
    /// All collections use sorted Vecs for deterministic iteration.
    pub fn adjudicate(&mut self, order_sets: &[OrderSet]) -> AdjudicationReport {
        let turn = self.state.current_turn;

        // Flatten all orders into a sorted list by unit_id for determinism
        let mut all_orders: Vec<_> = order_sets.iter().flat_map(|os| os.orders.iter()).collect();
        all_orders.sort_by_key(|o| o.unit_id);

        // Compute support counts as sorted Vec of ((unit_id, target), count)
        let mut support_entries: Vec<((UnitId, TerritoryId), u32)> = Vec::new();
        for order in &all_orders {
            if let OrderKind::Support {
                supported_unit_id,
                target,
            } = &order.kind
            {
                let key = (*supported_unit_id, *target);
                match support_entries.binary_search_by_key(&key, |e| e.0) {
                    Ok(i) => support_entries[i].1 += 1,
                    Err(i) => support_entries.insert(i, (key, 1)),
                }
            }
        }

        // Compute move intents sorted by (target, unit_id) for deterministic grouping
        // Each entry: (target, unit_id, strength)
        let mut move_intents: Vec<(TerritoryId, UnitId, u32)> = Vec::new();
        for order in &all_orders {
            if let OrderKind::Move { target } = &order.kind {
                let support = Self::lookup_support(&support_entries, (order.unit_id, *target));
                let strength = 1 + support;
                move_intents.push((*target, order.unit_id, strength));
            }
        }
        move_intents.sort_by_key(|&(t, uid, _)| (t, uid));

        // For each unique target, resolve who (if anyone) successfully moves there.
        // Result: sorted Vec of (unit_id, target) for successful movers.
        let mut successful_moves: Vec<(UnitId, TerritoryId)> = Vec::new();

        // Process each target group in sorted order
        let mut i = 0;
        while i < move_intents.len() {
            let target = move_intents[i].0;
            // Collect all attackers for this target
            let mut j = i;
            while j < move_intents.len() && move_intents[j].0 == target {
                j += 1;
            }
            let attackers = &move_intents[i..j];
            i = j;

            // Sort attackers by strength desc, then unit_id asc for determinism
            let mut sorted_attackers: Vec<(UnitId, u32)> =
                attackers.iter().map(|&(_, uid, s)| (uid, s)).collect();
            sorted_attackers.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

            let best_strength = sorted_attackers[0].1;
            let best_unit = sorted_attackers[0].0;

            // Tie check: if second attacker has equal strength, nobody moves
            let is_unique_best =
                sorted_attackers.len() == 1 || sorted_attackers[1].1 < best_strength;
            if !is_unique_best {
                continue;
            }

            // Find the defender currently at target
            let defender = self.state.unit_at(target);
            // Is the defender moving away from target?
            let defender_is_moving = defender
                .map(|d| {
                    move_intents
                        .iter()
                        .any(|&(t, uid, _)| uid == d.id && t != target)
                })
                .unwrap_or(false);

            let defender_strength = if let Some(def) = defender {
                if !defender_is_moving {
                    let def_support =
                        Self::lookup_support(&support_entries, (def.id, def.territory_id));
                    1 + def_support
                } else {
                    0
                }
            } else {
                0
            };

            if best_strength > defender_strength {
                // Insert into successful_moves sorted by unit_id
                let pos = successful_moves.partition_point(|(uid, _)| *uid < best_unit);
                successful_moves.insert(pos, (best_unit, target));
            }
        }

        // Build resolution steps (iterate all_orders which is sorted by unit_id)
        let mut steps: Vec<ResolutionStep> = Vec::new();
        for order in &all_orders {
            let unit = match self.state.unit_by_id(order.unit_id) {
                Some(u) => u,
                None => continue,
            };
            let from = unit.territory_id;

            match &order.kind {
                OrderKind::Hold => {
                    let hold_support =
                        Self::lookup_support(&support_entries, (order.unit_id, from));
                    steps.push(ResolutionStep {
                        turn,
                        unit_id: order.unit_id,
                        from,
                        to: from,
                        strength: 1 + hold_support,
                        outcome: ResolutionOutcome::Stayed,
                    });
                }
                OrderKind::Move { target } => {
                    let support = Self::lookup_support(&support_entries, (order.unit_id, *target));
                    let strength = 1 + support;
                    // Check if this unit is in successful_moves (sorted by unit_id)
                    let moved = successful_moves
                        .binary_search_by_key(&order.unit_id, |(uid, _)| *uid)
                        .ok()
                        .map(|idx| successful_moves[idx].1);

                    if let Some(dest) = moved {
                        steps.push(ResolutionStep {
                            turn,
                            unit_id: order.unit_id,
                            from,
                            to: dest,
                            strength,
                            outcome: ResolutionOutcome::Moved,
                        });
                    } else {
                        steps.push(ResolutionStep {
                            turn,
                            unit_id: order.unit_id,
                            from,
                            to: from,
                            strength,
                            outcome: ResolutionOutcome::Bounced,
                        });
                    }
                }
                OrderKind::Support { .. } => {
                    steps.push(ResolutionStep {
                        turn,
                        unit_id: order.unit_id,
                        from,
                        to: from,
                        strength: 1,
                        outcome: ResolutionOutcome::Supported,
                    });
                }
            }
        }

        // Apply successful moves to state (sorted by unit_id for determinism)
        for &(uid, dest) in &successful_moves {
            if let Some(unit) = self.state.units.iter_mut().find(|u| u.id == uid) {
                unit.territory_id = dest;
            }
        }

        // Advance turn
        self.state.current_turn = self.state.current_turn.next();

        // Steps are already in unit_id order (all_orders was sorted)
        AdjudicationReport { turn, steps }
    }
}
