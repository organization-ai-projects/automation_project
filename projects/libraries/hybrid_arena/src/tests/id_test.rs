// projects/libraries/hybrid_arena/src/tests/id_test.rs
use crate::Id;

#[test]
fn test_pack_unpack() {
    let id: Id<()> = Id::new(0xDEAD_BEEF, 0xCAFE_BABE);
    assert_eq!(id.index(), 0xDEAD_BEEF);
    assert_eq!(id.generation(), 0xCAFE_BABE);
}

#[test]
fn test_next_generation() {
    let id: Id<()> = Id::new(42, 0);
    let next = id.next_generation();
    assert_eq!(next.index(), 42);
    assert_eq!(next.generation(), 1);
}

#[test]
fn test_generation_wrapping() {
    let id: Id<()> = Id::new(0, u32::MAX);
    let next = id.next_generation();
    assert_eq!(next.generation(), 0);
}

#[test]
fn test_raw_roundtrip() {
    let id: Id<()> = Id::new(123, 456);
    let raw = id.to_raw();
    let restored: Id<()> = Id::from_raw(raw);
    assert_eq!(id, restored);
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Id<()>>(), 8);
}
