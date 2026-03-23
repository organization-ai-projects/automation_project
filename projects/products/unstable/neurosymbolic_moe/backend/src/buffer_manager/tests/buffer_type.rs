use crate::buffer_manager::BufferType;

#[test]
fn buffer_type_variants_are_constructible() {
    let working = BufferType::Working;
    let session = BufferType::Session;
    assert!(matches!(working, BufferType::Working));
    assert!(matches!(session, BufferType::Session));
}
