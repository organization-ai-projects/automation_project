use super::order_kind::OrderKind;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawOrder {
    pub unit_id: u32,
    pub kind: OrderKind,
}
