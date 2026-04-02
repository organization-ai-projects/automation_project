use crate::ui_app::run;

#[test]
fn run_symbol_exists() {
    let symbol = std::any::type_name_of_val(&run);
    assert!(symbol.contains("run"));
}
