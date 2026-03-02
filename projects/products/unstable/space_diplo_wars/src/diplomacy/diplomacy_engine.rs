use std::collections::BTreeMap;

use crate::diagnostics::error::SpaceDiploWarsError;
use crate::model::sim_state::SimState;
use crate::orders::order::Order;

use super::treaty::Treaty;
use super::treaty_id::TreatyId;
use super::treaty_kind::TreatyKind;

pub struct DiplomacyEngine;

impl DiplomacyEngine {
    /// Validate a diplomacy-related order. Returns Ok if the order is valid.
    pub fn validate_action(order: &Order, state: &SimState) -> Result<(), SpaceDiploWarsError> {
        use crate::orders::order_kind::OrderKind;
        match &order.kind {
            OrderKind::AcceptTreaty => {
                let treaty_id = order.params.get("treaty_id").ok_or_else(|| {
                    SpaceDiploWarsError::InvalidOrders("AcceptTreaty missing treaty_id".into())
                })?;
                // Offer must exist as a pending treaty
                if !state.treaties.contains_key(treaty_id.as_str()) {
                    // Treat as valid (offer may arrive same turn)
                }
                Ok(())
            }
            OrderKind::RejectTreaty | OrderKind::OfferTreaty | OrderKind::Embargo => Ok(()),
            _ => Ok(()),
        }
    }

    /// Apply all diplomacy orders for a turn, mutating state.
    /// Tie-breaker: orders sorted by empire_id then order_id (ascending string order).
    pub fn apply_turn(orders: &[Order], state: &mut SimState, current_turn: u64) -> Vec<String> {
        let mut events = Vec::new();
        let mut sorted_orders: Vec<&Order> = orders.iter().collect();
        // Tie-breaker: sort by empire_id then order_id
        sorted_orders.sort_by(|a, b| a.empire_id.0.cmp(&b.empire_id.0).then(a.id.0.cmp(&b.id.0)));

        use crate::orders::order_kind::OrderKind;
        // Collect offers first, indexed by a canonical treaty id
        let mut pending_offers: BTreeMap<String, (String, TreatyKind, Option<u64>)> =
            BTreeMap::new();

        for order in &sorted_orders {
            match &order.kind {
                OrderKind::OfferTreaty => {
                    let target = order.params.get("target").cloned().unwrap_or_default();
                    let kind_str = order.params.get("treaty_kind").cloned().unwrap_or_default();
                    let kind = parse_treaty_kind(&kind_str);
                    let end_turn = order
                        .params
                        .get("end_turn")
                        .and_then(|s| s.parse::<u64>().ok());
                    // Canonical treaty id: sorted empire ids + offer turn
                    let mut ids = vec![order.empire_id.0.clone(), target.clone()];
                    ids.sort();
                    let treaty_key = format!("treaty_{}_{}", ids[0], ids[1]);
                    let versioned_key = format!("{}_{}", treaty_key, current_turn);
                    pending_offers.insert(versioned_key.clone(), (target.clone(), kind, end_turn));
                    events.push(format!("TreatyOffered:{}", versioned_key));
                }
                OrderKind::AcceptTreaty => {
                    let treaty_id = order.params.get("treaty_id").cloned().unwrap_or_default();
                    if let Some((target, kind, end_turn)) = pending_offers.get(&treaty_id) {
                        let treaty = Treaty {
                            id: TreatyId(treaty_id.clone()),
                            kind: *kind,
                            parties: {
                                let mut v = vec![order.empire_id.clone()];
                                v.push(crate::model::empire_id::EmpireId(target.clone()));
                                v.sort();
                                v
                            },
                            start_turn: current_turn,
                            end_turn: *end_turn,
                            rules: BTreeMap::new(),
                        };
                        state.treaties.insert(treaty_id.clone(), treaty);
                        events.push(format!("TreatyAccepted:{}", treaty_id));
                    }
                }
                OrderKind::RejectTreaty => {
                    let treaty_id = order.params.get("treaty_id").cloned().unwrap_or_default();
                    events.push(format!("TreatyRejected:{}", treaty_id));
                }
                _ => {}
            }
        }

        // Remove expired treaties
        let expired: Vec<String> = state
            .treaties
            .iter()
            .filter(|(_, t)| t.end_turn.map_or(false, |e| e < current_turn))
            .map(|(k, _)| k.clone())
            .collect();
        for key in expired {
            state.treaties.remove(&key);
            events.push(format!("TreatyExpired:{}", key));
        }

        events
    }
}

fn parse_treaty_kind(s: &str) -> TreatyKind {
    match s {
        "Alliance" => TreatyKind::Alliance,
        "NonAggression" => TreatyKind::NonAggression,
        _ => TreatyKind::TradePact,
    }
}
