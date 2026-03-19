use crate::components::card_components::{login_card, setup_card, users_table};

#[test]
fn card_component_symbols_are_exposed() {
    let setup_symbol = std::any::type_name_of_val(&setup_card);
    let login_symbol = std::any::type_name_of_val(&login_card);
    let table_symbol = std::any::type_name_of_val(&users_table);

    assert!(setup_symbol.contains("setup_card"));
    assert!(login_symbol.contains("login_card"));
    assert!(table_symbol.contains("users_table"));
}
