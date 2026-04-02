use crate::finance::ledger::Ledger;

pub struct FinanceEngine;

impl FinanceEngine {
    pub fn summarize(ledger: &Ledger) -> FinanceSummary {
        FinanceSummary {
            total_revenue: ledger.total_revenue(),
            total_costs: ledger.total_costs(),
            net_profit: ledger.net_profit(),
            transaction_count: ledger.transactions().len() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FinanceSummary {
    pub total_revenue: i64,
    pub total_costs: i64,
    pub net_profit: i64,
    pub transaction_count: u64,
}
