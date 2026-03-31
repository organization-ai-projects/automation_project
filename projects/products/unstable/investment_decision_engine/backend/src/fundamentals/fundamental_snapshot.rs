use serde::{Deserialize, Serialize};

use crate::fundamentals::{BalanceSheet, CashFlowStatement, IncomeStatement};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundamentalSnapshot {
    pub ticker: String,
    pub balance_sheets: Vec<BalanceSheet>,
    pub income_statements: Vec<IncomeStatement>,
    pub cash_flow_statements: Vec<CashFlowStatement>,
}

impl FundamentalSnapshot {
    pub fn new(ticker: impl Into<String>) -> Self {
        Self {
            ticker: ticker.into(),
            balance_sheets: Vec::new(),
            income_statements: Vec::new(),
            cash_flow_statements: Vec::new(),
        }
    }

    pub fn latest_income(&self) -> Option<&IncomeStatement> {
        self.income_statements.last()
    }

    pub fn latest_balance(&self) -> Option<&BalanceSheet> {
        self.balance_sheets.last()
    }

    pub fn latest_cash_flow(&self) -> Option<&CashFlowStatement> {
        self.cash_flow_statements.last()
    }
}
