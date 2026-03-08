use super::raw_order_set::RawOrderSet;
use crate::diagnostics::diplo_sim_error::DiploSimError;
use crate::model::faction_id::FactionId;
use crate::model::unit_id::UnitId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;

pub fn parse_order_set_from_str(
    json: &str,
    next_order_id: &mut u32,
) -> Result<OrderSet, DiploSimError> {
    let raw: RawOrderSet = common_json::from_str(json)
        .map_err(|e| DiploSimError::Io(format!("JSON parse error: {e}")))?;

    let order_kinds: Vec<(UnitId, OrderKind)> = raw
        .orders
        .into_iter()
        .map(|ro| (UnitId(ro.unit_id), ro.kind))
        .collect();

    Ok(OrderSet::from_raw(
        FactionId(raw.faction_id),
        order_kinds,
        next_order_id,
    ))
}
