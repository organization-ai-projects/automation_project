use crate::finance::transaction::Transaction;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn supply_cost_is_negative() {
    let t = Transaction::supply_cost(Tick(0), StoreId(0), 500);
    assert!(t.amount < 0);
}

#[test]
fn sale_is_positive() {
    let t = Transaction::sale(Tick(0), StoreId(0), 1000);
    assert!(t.amount > 0);
}
