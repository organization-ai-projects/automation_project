use crate::debate::debate_resolver::DebateResolver;

#[test]
fn debate_resolver_type_is_constructible() {
    let resolver = DebateResolver;
    let resolver_type = std::any::type_name_of_val(&resolver);
    assert!(resolver_type.contains("DebateResolver"));
}
