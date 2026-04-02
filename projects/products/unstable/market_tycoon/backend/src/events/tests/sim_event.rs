use crate::events::sim_event::SimEvent;
use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::pricing::price::Price;
use crate::time::tick::Tick;

#[test]
fn delivery_event() {
    let e = SimEvent::delivery(Tick(5), StoreId(1), Good::Widget, 20);
    assert_eq!(e.tick, Tick(5));
}

#[test]
fn price_updated_event() {
    let e = SimEvent::price_updated(Tick(1), StoreId(0), Good::Gadget, Price::new(1000));
    assert_eq!(e.tick, Tick(1));
}

#[test]
fn sale_event() {
    let e = SimEvent::sale(Tick(3), StoreId(0), Good::Widget, 5, 2500);
    assert_eq!(e.tick, Tick(3));
}
