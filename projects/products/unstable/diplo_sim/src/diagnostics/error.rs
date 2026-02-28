use thiserror::Error;
use crate::map::territory_id::TerritoryId;
use crate::model::unit_id::UnitId;
use crate::orders::order_id::OrderId;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DiploSimError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Map validation error: {0}")]
    MapValidation(String),
    #[error("Order validation error on order {order_id}: unit {unit_id} territory {territory_id} - {reason}")]
    OrderValidation {
        order_id: OrderId,
        unit_id: UnitId,
        territory_id: TerritoryId,
        reason: String,
    },
    #[error("Replay error: {0}")]
    Replay(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Config error: {0}")]
    Config(String),
}
