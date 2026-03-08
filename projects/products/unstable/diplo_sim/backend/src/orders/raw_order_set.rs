use super::raw_order::RawOrder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawOrderSet {
    pub faction_id: u32,
    pub orders: Vec<RawOrder>,
}
