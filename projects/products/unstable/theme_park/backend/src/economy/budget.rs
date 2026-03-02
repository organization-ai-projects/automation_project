#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// The park's financial state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub balance: i64,
    pub total_revenue: i64,
    pub total_expenses: i64,
}

impl Budget {
    pub fn new(initial: i64) -> Self {
        Self {
            balance: initial,
            total_revenue: 0,
            total_expenses: 0,
        }
    }

    pub fn add_revenue(&mut self, amount: u32) {
        let a = amount as i64;
        self.balance += a;
        self.total_revenue += a;
    }

    pub fn add_expense(&mut self, amount: u32) {
        let a = amount as i64;
        self.balance -= a;
        self.total_expenses += a;
    }
}
