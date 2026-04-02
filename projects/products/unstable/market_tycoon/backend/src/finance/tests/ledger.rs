use crate::finance::ledger::Ledger;
use crate::finance::transaction::Transaction;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn ledger_tracks_profit() {
    let mut l = Ledger::new();
    l.record(Transaction::sale(Tick(0), StoreId(0), 2000));
    l.record(Transaction::supply_cost(Tick(0), StoreId(0), 500));
    assert_eq!(l.total_revenue(), 2000);
    assert_eq!(l.total_costs(), -500);
    assert_eq!(l.net_profit(), 1500);
}
