use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::pricing::price::Price;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    store_id: StoreId,
    stock: BTreeMap<Good, u64>,
    prices: BTreeMap<Good, Price>,
}

impl Inventory {
    pub fn new(store_id: StoreId) -> Self {
        Self {
            store_id,
            stock: BTreeMap::new(),
            prices: BTreeMap::new(),
        }
    }

    pub fn store_id(&self) -> StoreId {
        self.store_id
    }

    pub fn stock(&self) -> &BTreeMap<Good, u64> {
        &self.stock
    }

    pub fn prices(&self) -> &BTreeMap<Good, Price> {
        &self.prices
    }

    pub fn add_stock(&mut self, good: Good, quantity: u64) {
        *self.stock.entry(good).or_insert(0) += quantity;
    }

    pub fn remove_stock(&mut self, good: Good, quantity: u64) -> bool {
        if let Some(current) = self.stock.get_mut(&good) {
            if *current >= quantity {
                *current -= quantity;
                return true;
            }
        }
        false
    }

    pub fn get_stock(&self, good: &Good) -> u64 {
        self.stock.get(good).copied().unwrap_or(0)
    }

    pub fn set_price(&mut self, good: Good, price: Price) {
        self.prices.insert(good, price);
    }

    pub fn get_price(&self, good: &Good) -> Option<Price> {
        self.prices.get(good).copied()
    }
}
