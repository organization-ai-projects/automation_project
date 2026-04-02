use crate::pricing::price::Price;

#[test]
fn price_display() {
    assert_eq!(format!("{}", Price::new(1550)), "$15.50");
}

#[test]
fn price_cents() {
    assert_eq!(Price::new(999).cents(), 999);
}

#[test]
fn price_ordering() {
    assert!(Price::new(100) < Price::new(200));
}
