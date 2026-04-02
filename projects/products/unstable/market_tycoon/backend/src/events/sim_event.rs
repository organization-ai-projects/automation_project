use serde::{Deserialize, Serialize};

use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::pricing::price::Price;
use crate::time::tick::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub tick: Tick,
    pub kind: SimEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEventKind {
    Delivery {
        store_id: StoreId,
        good: Good,
        quantity: u64,
    },
    PriceUpdated {
        store_id: StoreId,
        good: Good,
        price: Price,
    },
    Sale {
        store_id: StoreId,
        good: Good,
        quantity: u64,
        revenue: i64,
    },
}

impl SimEvent {
    pub fn delivery(tick: Tick, store_id: StoreId, good: Good, quantity: u64) -> Self {
        Self {
            tick,
            kind: SimEventKind::Delivery {
                store_id,
                good,
                quantity,
            },
        }
    }

    pub fn price_updated(tick: Tick, store_id: StoreId, good: Good, price: Price) -> Self {
        Self {
            tick,
            kind: SimEventKind::PriceUpdated {
                store_id,
                good,
                price,
            },
        }
    }

    pub fn sale(tick: Tick, store_id: StoreId, good: Good, quantity: u64, revenue: i64) -> Self {
        Self {
            tick,
            kind: SimEventKind::Sale {
                store_id,
                good,
                quantity,
                revenue,
            },
        }
    }
}
