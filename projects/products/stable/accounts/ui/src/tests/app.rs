use crate::app::app;

#[test]
fn app_component_symbol_is_exposed() {
    let symbol = std::any::type_name_of_val(&app);
    assert!(symbol.contains("app"));
}
