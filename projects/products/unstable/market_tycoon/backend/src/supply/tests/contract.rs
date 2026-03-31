use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::supply::contract::Contract;
use crate::supply::supplier::SupplierId;

#[test]
fn contract_fields() {
    let c = Contract {
        supplier_id: SupplierId(1),
        store_id: StoreId(0),
        good: Good::Widget,
        quantity_per_delivery: 50,
        delivery_interval: 5,
        cost_per_unit: 300,
    };
    assert_eq!(c.quantity_per_delivery, 50);
    assert_eq!(c.delivery_interval, 5);
}
