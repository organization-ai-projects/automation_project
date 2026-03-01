// projects/products/unstable/hospital_tycoon/backend/src/economy/budget.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub balance: i64,
    pub income: i64,
    pub expenses: i64,
}

impl Budget {
    pub fn new(initial_balance: i64) -> Self {
        Self {
            balance: initial_balance,
            income: 0,
            expenses: 0,
        }
    }

    pub fn add_income(&mut self, amount: i64) {
        self.balance += amount;
        self.income += amount;
    }

    pub fn add_expense(&mut self, amount: i64) {
        self.balance -= amount;
        self.expenses += amount;
    }
}
