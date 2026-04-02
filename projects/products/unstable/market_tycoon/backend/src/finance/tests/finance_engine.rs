use crate::finance::finance_engine::FinanceEngine;
use crate::finance::ledger::Ledger;
use crate::finance::transaction::Transaction;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn summarize_empty_ledger() {
    let l = Ledger::new();
    let s = FinanceEngine::summarize(&l);
    assert_eq!(s.total_revenue, 0);
    assert_eq!(s.net_profit, 0);
    assert_eq!(s.transaction_count, 0);
}
