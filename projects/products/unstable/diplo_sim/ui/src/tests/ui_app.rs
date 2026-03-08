use crate::ui_app::product_name;

#[test]
fn product_name_matches_branding() {
    assert_eq!(product_name(), "Diplo Sim");
}
