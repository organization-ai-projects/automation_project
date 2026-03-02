use crate::diplomacy::diplomacy_engine::DiplomacyEngine;
use crate::economy::economy_engine::EconomyEngine;
use crate::events::game_event::GameEvent;
use crate::model::sim_state::SimState;
use crate::orders::order::Order;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_validator::OrderValidator;
use crate::war::battle_input::BattleInput;
use crate::war::battle_resolver::BattleResolver;

use super::resolution_report::ResolutionReport;

/// The sole place where simultaneous strategic outcomes are applied.
pub struct ResolutionEngine;

impl ResolutionEngine {
    /// Resolve a full turn:
    /// 1. ValidateOrders  - check all orders; skip invalid ones (log errors)
    /// 2. ApplyDiplomacy  - process treaty offers/accepts/rejects (sorted by empire_id, then order_id)
    /// 3. ResolveCombat   - deterministic battle resolution; tie-breaker = attacker empire_id string
    /// 4. UpdateEconomy   - apply economy tick
    /// 5. EmitEvents      - collect all events into ResolutionReport
    pub fn resolve_turn(state: &mut SimState, orders: &[Order], turn: u64) -> ResolutionReport {
        let mut report = ResolutionReport::new(turn);

        // 1. Validate orders; keep only valid ones (sort for determinism: empire_id then order_id)
        let mut sorted_orders: Vec<&Order> = orders.iter().collect();
        sorted_orders.sort_by(|a, b| a.empire_id.0.cmp(&b.empire_id.0).then(a.id.0.cmp(&b.id.0)));

        let mut valid_orders: Vec<&Order> = Vec::new();
        for order in &sorted_orders {
            match OrderValidator::validate(order, state) {
                Ok(()) => valid_orders.push(order),
                Err(e) => report.validation_errors.push(e.to_string()),
            }
        }

        // 2. Apply diplomacy
        let diplo_orders: Vec<Order> = valid_orders
            .iter()
            .filter(|o| {
                matches!(
                    o.kind,
                    OrderKind::OfferTreaty
                        | OrderKind::AcceptTreaty
                        | OrderKind::RejectTreaty
                        | OrderKind::Embargo
                )
            })
            .map(|o| (*o).clone())
            .collect();
        let diplo_events = DiplomacyEngine::apply_turn(&diplo_orders, state, turn);
        report.diplomacy_events = diplo_events;

        // 3. Resolve combat: find attack orders and build BattleInputs
        // Collect attack orders, sorted by empire_id then order_id (already sorted)
        let attack_orders: Vec<&Order> = valid_orders
            .iter()
            .filter(|o| o.kind == OrderKind::AttackFleet)
            .copied()
            .collect();

        for order in attack_orders {
            let attacker_fleet_id_str = match order.params.get("fleet_id") {
                Some(s) => s.clone(),
                None => continue,
            };
            let target_fleet_id_str = match order.params.get("target_fleet") {
                Some(s) => s.clone(),
                None => continue,
            };
            let system = order.params.get("system").cloned().unwrap_or_default();

            let attacker_fleet_id = crate::model::fleet_id::FleetId(attacker_fleet_id_str.clone());
            let target_fleet_id = crate::model::fleet_id::FleetId(target_fleet_id_str.clone());

            let attacker = match state.fleets.get(&attacker_fleet_id) {
                Some(f) => f.clone(),
                None => continue,
            };
            let defender = match state.fleets.get(&target_fleet_id) {
                Some(f) => f.clone(),
                None => continue,
            };

            let input = BattleInput {
                attacker,
                defender,
                location: crate::map::star_system_id::StarSystemId(system),
            };
            let battle_report = BattleResolver::resolve(input);

            // Apply battle result: remove losing fleet's ships
            if battle_report.attacker_wins {
                state.fleets.remove(&target_fleet_id);
            } else {
                state.fleets.remove(&attacker_fleet_id);
            }

            report.battles.push(battle_report);
        }

        // Apply MoveFleet orders
        let move_orders: Vec<&Order> = valid_orders
            .iter()
            .filter(|o| o.kind == OrderKind::MoveFleet)
            .copied()
            .collect();

        for order in move_orders {
            let fleet_id_str = match order.params.get("fleet_id") {
                Some(s) => s.clone(),
                None => continue,
            };
            let dest = match order.params.get("destination") {
                Some(s) => s.clone(),
                None => continue,
            };
            let fleet_id = crate::model::fleet_id::FleetId(fleet_id_str);
            if let Some(fleet) = state.fleets.get_mut(&fleet_id) {
                fleet.location = dest.clone();
                fleet.destination = Some(dest);
            }
        }

        // 4. Update economy
        EconomyEngine::tick(state);

        // 5. Emit events
        report.game_events.push(GameEvent::TurnResolved { turn });

        report
    }
}
