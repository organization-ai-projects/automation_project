use crate::model::good::Good;
use crate::model::inventory::Inventory;
use crate::model::store_id::StoreId;
use crate::pricing::price::Price;

#[test]
fn add_and_get_stock() {
    let mut inv = Inventory::new(StoreId(1));
    inv.add_stock(Good::Widget, 10);
    assert_eq!(inv.get_stock(&Good::Widget), 10);
    inv.add_stock(Good::Widget, 5);
    assert_eq!(inv.get_stock(&Good::Widget), 15);
}

#[test]
fn remove_stock_success() {
    let mut inv = Inventory::new(StoreId(1));
    inv.add_stock(Good::Widget, 10);
    assert!(inv.remove_stock(Good::Widget, 5));
    assert_eq!(inv.get_stock(&Good::Widget), 5);
}

#[test]
fn remove_stock_insufficient() {
    let mut inv = Inventory::new(StoreId(1));
    inv.add_stock(Good::Widget, 3);
    assert!(!inv.remove_stock(Good::Widget, 5));
    assert_eq!(inv.get_stock(&Good::Widget), 3);
}

#[test]
fn set_and_get_price() {
    let mut inv = Inventory::new(StoreId(1));
    inv.set_price(Good::Widget, Price::new(1500));
    assert_eq!(inv.get_price(&Good::Widget), Some(Price::new(1500)));
}
