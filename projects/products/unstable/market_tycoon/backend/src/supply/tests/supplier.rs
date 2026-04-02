use crate::supply::supplier::{Supplier, SupplierId};

#[test]
fn supplier_creation() {
    let s = Supplier::new(SupplierId(1), "WidgetCo".into());
    assert_eq!(s.id(), SupplierId(1));
    assert_eq!(s.name(), "WidgetCo");
}

#[test]
fn supplier_id_display() {
    assert_eq!(format!("{}", SupplierId(3)), "supplier-3");
}
