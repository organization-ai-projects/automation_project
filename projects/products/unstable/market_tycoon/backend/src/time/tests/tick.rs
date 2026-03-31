use crate::time::tick::Tick;

#[test]
fn tick_value() {
    let t = Tick(5);
    assert_eq!(t.value(), 5);
}

#[test]
fn tick_display() {
    let t = Tick(42);
    assert_eq!(format!("{t}"), "t42");
}

#[test]
fn tick_ordering() {
    assert!(Tick(1) < Tick(2));
    assert_eq!(Tick(3), Tick(3));
}
