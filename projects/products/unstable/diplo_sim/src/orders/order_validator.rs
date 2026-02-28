use crate::diagnostics::error::DiploSimError;
use crate::map::map_graph::MapGraph;
use crate::model::game_state::GameState;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;

/// Validates all orders in an OrderSet against the current game state and map.
/// Returns a list of validation errors; empty means all orders are valid.
pub fn validate_order_set(
    order_set: &OrderSet,
    state: &GameState,
    map: &MapGraph,
) -> Vec<DiploSimError> {
    let mut errors = Vec::new();

    for order in &order_set.orders {
        // Verify the unit exists
        let unit = match state.unit_by_id(order.unit_id) {
            Some(u) => u,
            None => {
                errors.push(DiploSimError::OrderValidation {
                    order_id: order.id,
                    unit_id: order.unit_id,
                    territory_id: crate::map::territory_id::TerritoryId(0),
                    reason: format!("Unit {} does not exist", order.unit_id.0),
                });
                continue;
            }
        };

        // Verify the unit belongs to the faction issuing the order
        if unit.faction_id != order_set.faction_id {
            errors.push(DiploSimError::OrderValidation {
                order_id: order.id,
                unit_id: order.unit_id,
                territory_id: unit.territory_id,
                reason: format!(
                    "Unit {} belongs to faction {}, not {}",
                    order.unit_id.0, unit.faction_id.0, order_set.faction_id.0
                ),
            });
            continue;
        }

        match &order.kind {
            OrderKind::Hold => {}
            OrderKind::Move { target } => {
                if !map.territory_exists(*target) {
                    errors.push(DiploSimError::OrderValidation {
                        order_id: order.id,
                        unit_id: order.unit_id,
                        territory_id: *target,
                        reason: format!("Target territory {} does not exist", target.0),
                    });
                } else if !map.is_adjacent(unit.territory_id, *target) {
                    errors.push(DiploSimError::OrderValidation {
                        order_id: order.id,
                        unit_id: order.unit_id,
                        territory_id: *target,
                        reason: format!(
                            "Territory {} is not adjacent to {}",
                            target.0, unit.territory_id.0
                        ),
                    });
                }
            }
            OrderKind::Support {
                supported_unit_id,
                target,
            } => {
                if state.unit_by_id(*supported_unit_id).is_none() {
                    errors.push(DiploSimError::OrderValidation {
                        order_id: order.id,
                        unit_id: order.unit_id,
                        territory_id: unit.territory_id,
                        reason: format!("Supported unit {} does not exist", supported_unit_id.0),
                    });
                }
                if !map.territory_exists(*target) {
                    errors.push(DiploSimError::OrderValidation {
                        order_id: order.id,
                        unit_id: order.unit_id,
                        territory_id: *target,
                        reason: format!("Support target territory {} does not exist", target.0),
                    });
                }
            }
        }
    }

    errors
}
