use crate::diagnostics::error::SpaceDiploWarsError;
use crate::model::sim_state::SimState;

use super::order::Order;

pub struct OrderValidator;

impl OrderValidator {
    /// Validate a single order against the current state.
    /// Returns Ok(()) if valid, Err with explanation otherwise.
    pub fn validate(order: &Order, state: &SimState) -> Result<(), SpaceDiploWarsError> {
        use crate::orders::order_kind::OrderKind;

        // Empire must exist in state
        if !state.empires.contains_key(&order.empire_id) {
            return Err(SpaceDiploWarsError::InvalidOrders(format!(
                "Unknown empire: {}",
                order.empire_id.0
            )));
        }

        match &order.kind {
            OrderKind::MoveFleet | OrderKind::AttackFleet => {
                let fleet_id_str = order.params.get("fleet_id").ok_or_else(|| {
                    SpaceDiploWarsError::InvalidOrders("MoveFleet/AttackFleet requires fleet_id param".into())
                })?;
                let fleet_id = crate::model::fleet_id::FleetId(fleet_id_str.clone());
                let fleet = state.fleets.get(&fleet_id).ok_or_else(|| {
                    SpaceDiploWarsError::InvalidOrders(format!("Unknown fleet: {}", fleet_id_str))
                })?;
                if fleet.empire_id != order.empire_id {
                    return Err(SpaceDiploWarsError::InvalidOrders(
                        "Fleet does not belong to ordering empire".into(),
                    ));
                }
            }
            OrderKind::DefendSystem | OrderKind::Invest => {
                // system param required
                if order.params.get("system").is_none() && order.params.get("fleet_id").is_none() {
                    return Err(SpaceDiploWarsError::InvalidOrders(
                        "DefendSystem/Invest requires system param".into(),
                    ));
                }
            }
            OrderKind::OfferTreaty => {
                order.params.get("target").ok_or_else(|| {
                    SpaceDiploWarsError::InvalidOrders("OfferTreaty requires target param".into())
                })?;
            }
            OrderKind::AcceptTreaty | OrderKind::RejectTreaty => {
                order.params.get("treaty_id").ok_or_else(|| {
                    SpaceDiploWarsError::InvalidOrders(
                        "AcceptTreaty/RejectTreaty requires treaty_id param".into(),
                    )
                })?;
            }
            OrderKind::Embargo => {}
        }
        Ok(())
    }

    /// Validate all orders in a slice, returning a list of errors (empty = all valid).
    pub fn validate_all(orders: &[Order], state: &SimState) -> Vec<SpaceDiploWarsError> {
        orders
            .iter()
            .filter_map(|o| Self::validate(o, state).err())
            .collect()
    }
}
