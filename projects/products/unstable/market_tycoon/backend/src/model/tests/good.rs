use crate::model::good::Good;

#[test]
fn display() {
    assert_eq!(format!("{}", Good::Widget), "Widget");
    assert_eq!(format!("{}", Good::Gadget), "Gadget");
    assert_eq!(format!("{}", Good::Gizmo), "Gizmo");
}

#[test]
fn ordering() {
    assert!(Good::Gadget < Good::Gizmo);
}
