#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// View of economy state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyScreen {
    pub balance: i64,
    pub total_revenue: i64,
    pub total_expenses: i64,
}

impl EconomyScreen {
    pub fn render(&self) -> String {
        format!(
            "[Economy] balance={} revenue={} expenses={}",
            self.balance, self.total_revenue, self.total_expenses
        )
    }
}
