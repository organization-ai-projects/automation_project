use std::collections::BTreeMap;

use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;
use crate::model::sim_state::SimState;
use crate::orders::order::Order;
use crate::orders::order_id::OrderId;
use crate::orders::order_kind::OrderKind;

use super::ai_profile::AiProfile;

pub struct AiEngine;

impl AiEngine {
    /// Generate orders for a given empire based on AiProfile and current state.
    /// Deterministic: no RNG, only state-driven decisions sorted by IDs.
    pub fn generate_orders(
        empire_id: &EmpireId,
        profile: &AiProfile,
        state: &SimState,
        turn: u64,
    ) -> Vec<Order> {
        // Deterministic baseline behavior while the full AI strategy matures.
        let empire_exists = state.empires.contains_key(empire_id);
        let same_or_future_turn = turn >= state.current_turn.0;
        let profile_is_valid = profile.aggression >= 0.0
            && profile.diplomacy_bias >= 0.0
            && profile.economic_focus >= 0.0;

        if !empire_exists || !same_or_future_turn || !profile_is_valid {
            return Vec::new();
        }

        let mut orders = Vec::new();
        let mut own_fleets: Vec<(FleetId, String)> = state
            .fleets
            .values()
            .filter(|fleet| fleet.empire_id == *empire_id)
            .map(|fleet| (fleet.id.clone(), fleet.location.clone()))
            .collect();
        own_fleets.sort_by(|a, b| a.0.0.cmp(&b.0.0));

        if let Some((fleet_id, location)) = own_fleets.first() {
            if profile.aggression >= 0.7 {
                let target = state
                    .fleets
                    .values()
                    .filter(|fleet| fleet.empire_id != *empire_id && fleet.location == *location)
                    .map(|fleet| fleet.id.clone())
                    .min_by(|a, b| a.0.cmp(&b.0));
                if let Some(target_fleet_id) = target {
                    let mut params = BTreeMap::new();
                    params.insert("fleet_id".to_string(), fleet_id.0.clone());
                    params.insert("target_fleet".to_string(), target_fleet_id.0.clone());
                    params.insert("system".to_string(), location.clone());
                    orders.push(Order {
                        id: OrderId(format!("ai_attack_{}_{}", empire_id.0, turn)),
                        empire_id: empire_id.clone(),
                        kind: OrderKind::AttackFleet,
                        params,
                    });
                    return orders;
                }
            }

            if profile.diplomacy_bias >= 0.7 && turn % 3 == 1 {
                let target_empire = state
                    .empires
                    .keys()
                    .filter(|candidate| **candidate != *empire_id)
                    .min_by(|a, b| a.0.cmp(&b.0));
                if let Some(target) = target_empire {
                    let mut params = BTreeMap::new();
                    params.insert("target".to_string(), target.0.clone());
                    params.insert("treaty_kind".to_string(), "TradePact".to_string());
                    orders.push(Order {
                        id: OrderId(format!("ai_treaty_{}_{}", empire_id.0, turn)),
                        empire_id: empire_id.clone(),
                        kind: OrderKind::OfferTreaty,
                        params,
                    });
                    return orders;
                }
            }

            let destination = state
                .star_map
                .routes
                .iter()
                .filter_map(|route| {
                    if route.from.0 == *location {
                        Some(route.to.0.clone())
                    } else if route.to.0 == *location {
                        Some(route.from.0.clone())
                    } else {
                        None
                    }
                })
                .min();

            if let Some(destination) = destination {
                let mut params = BTreeMap::new();
                params.insert("fleet_id".to_string(), fleet_id.0.clone());
                params.insert("destination".to_string(), destination);
                orders.push(Order {
                    id: OrderId(format!("ai_move_{}_{}", empire_id.0, turn)),
                    empire_id: empire_id.clone(),
                    kind: OrderKind::MoveFleet,
                    params,
                });
                return orders;
            }
        }

        if let Some(empire) = state.empires.get(empire_id) {
            let mut params = BTreeMap::new();
            params.insert("system".to_string(), empire.home_system.clone());
            orders.push(Order {
                id: OrderId(format!("ai_invest_{}_{}", empire_id.0, turn)),
                empire_id: empire_id.clone(),
                kind: OrderKind::Invest,
                params,
            });
        }

        orders
    }
}
