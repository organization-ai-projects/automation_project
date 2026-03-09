use crate::actions::action_resolver::ActionResolver;

#[test]
fn action_resolver_type_is_constructible() {
    let resolver = ActionResolver;
    let resolver_type = std::any::type_name_of_val(&resolver);
    assert!(resolver_type.contains("ActionResolver"));
}
