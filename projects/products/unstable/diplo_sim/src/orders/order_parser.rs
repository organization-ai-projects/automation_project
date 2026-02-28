use serde::Deserialize;
use crate::diagnostics::error::DiploSimError;
use crate::model::faction_id::FactionId;
use crate::model::unit_id::UnitId;
use crate::orders::order_id::OrderId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order::Order;
use crate::orders::order_set::OrderSet;

/// Raw JSON format for a single order (before assigning OrderId).
#[derive(Debug, Deserialize)]
struct RawOrder {
    unit_id: u32,
    kind: OrderKind,
}

/// Raw JSON format for an order set.
#[derive(Debug, Deserialize)]
struct RawOrderSet {
    faction_id: u32,
    orders: Vec<RawOrder>,
}

pub fn parse_order_set_from_str(json: &str, next_order_id: &mut u32) -> Result<OrderSet, DiploSimError> {
    let raw: RawOrderSet = common_json::from_str(json)
        .map_err(|e| DiploSimError::Io(format!("JSON parse error: {e}")))?;

    let orders: Vec<Order> = raw
        .orders
        .into_iter()
        .map(|ro| {
            let id = OrderId(*next_order_id);
            *next_order_id += 1;
            Order {
                id,
                unit_id: UnitId(ro.unit_id),
                kind: ro.kind,
            }
        })
        .collect();

    Ok(OrderSet {
        faction_id: FactionId(raw.faction_id),
        orders,
    })
}
