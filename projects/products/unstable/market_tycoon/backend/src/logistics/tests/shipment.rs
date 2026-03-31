use crate::logistics::shipment::Shipment;
use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn shipment_creation() {
    let s = Shipment::new(StoreId(0), StoreId(1), Good::Widget, 50, Tick(0), Tick(5));
    assert_eq!(s.origin, StoreId(0));
    assert_eq!(s.destination, StoreId(1));
    assert_eq!(s.quantity, 50);
}
