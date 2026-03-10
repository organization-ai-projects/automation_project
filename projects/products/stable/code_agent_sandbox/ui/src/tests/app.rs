use crate::app::launch;

#[test]
fn app_launch_symbol_exists() {
    let symbol = std::any::type_name_of_val(&launch);
    assert!(symbol.contains("launch"));
}
