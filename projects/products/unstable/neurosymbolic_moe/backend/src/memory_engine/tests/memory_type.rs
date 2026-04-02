use crate::memory_engine::MemoryType;

#[test]
fn memory_type_variants_are_constructible() {
    let short = MemoryType::Short;
    let medium = MemoryType::Medium;
    let long = MemoryType::Long;
    assert!(matches!(short, MemoryType::Short));
    assert!(matches!(medium, MemoryType::Medium));
    assert!(matches!(long, MemoryType::Long));
}
