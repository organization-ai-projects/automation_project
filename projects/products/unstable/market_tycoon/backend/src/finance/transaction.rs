use serde::{Deserialize, Serialize};

use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tick: Tick,
    pub store_id: StoreId,
    pub kind: TransactionKind,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionKind {
    SupplyCost,
    Sale,
}

impl Transaction {
    pub fn supply_cost(tick: Tick, store_id: StoreId, amount: i64) -> Self {
        Self {
            tick,
            store_id,
            kind: TransactionKind::SupplyCost,
            amount: -amount.abs(),
        }
    }

    pub fn sale(tick: Tick, store_id: StoreId, amount: i64) -> Self {
        Self {
            tick,
            store_id,
            kind: TransactionKind::Sale,
            amount,
        }
    }
}
