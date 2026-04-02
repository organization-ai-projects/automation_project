use serde::{Deserialize, Serialize};

use crate::finance::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn record(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn total_revenue(&self) -> i64 {
        self.transactions
            .iter()
            .filter(|t| t.amount > 0)
            .map(|t| t.amount)
            .sum()
    }

    pub fn total_costs(&self) -> i64 {
        self.transactions
            .iter()
            .filter(|t| t.amount < 0)
            .map(|t| t.amount)
            .sum()
    }

    pub fn net_profit(&self) -> i64 {
        self.transactions.iter().map(|t| t.amount).sum()
    }
}
