// projects/libraries/hybrid_arena/src/tests/slot_arena_test.rs
use crate::{Id, SlotArena};

use super::helpers::{ArenaTestHelpers, assert_empty, assert_len};

#[test]
fn test_alloc_and_get() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.test_alloc(10);
    let id2 = arena.test_alloc(20);

    assert_eq!(arena.get(id1), Some(&10));
    assert_eq!(arena.get(id2), Some(&20));
    assert_len(&arena, 2);
}

#[test]
fn test_remove_and_reuse() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.test_alloc(10);
    let id2 = arena.test_alloc(20);

    // Remove first item
    assert_eq!(arena.remove(id1), Some(10));
    assert!(arena.get(id1).is_none());
    assert_len(&arena, 1);

    // Allocate new item - should reuse slot 0
    let id3 = arena.test_alloc(30);
    assert_eq!(id3.index(), id1.index());
    assert_ne!(id3.generation(), id1.generation()); // Generation bumped
    assert_eq!(arena[id3], 30);
    assert_len(&arena, 2);

    // Old ID should still be invalid
    assert!(arena.get(id1).is_none());
    assert_eq!(arena.get(id2), Some(&20));
}

#[test]
fn test_generation_prevents_use_after_free() {
    let mut arena: SlotArena<String> = SlotArena::new();
    let id = arena.test_alloc("first".to_string());
    let old_gen = id.generation();

    arena.remove(id);
    let new_id = arena.test_alloc("second".to_string());

    // Same slot, different generation
    assert_eq!(id.index(), new_id.index());
    assert_eq!(new_id.generation(), old_gen.wrapping_add(1));

    // Old ID returns None
    assert!(arena.get(id).is_none());
    assert_eq!(arena.get(new_id), Some(&"second".to_string()));
}

#[test]
fn test_get2_mut() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.test_alloc(10);
    let id2 = arena.test_alloc(20);

    let (a, b) = arena.get_mut(id1, id2);
    let a = a.expect("id1 present");
    let b = b.expect("id2 present");
    *a += 1;
    *b += 1;

    assert_eq!(arena[id1], 11);
    assert_eq!(arena[id2], 21);
}

#[test]
#[should_panic(expected = "cannot borrow the same slot twice")]
fn test_get2_mut_same_id_panics() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id = arena.test_alloc(10);
    let _ = arena.get_mut(id, id);
}

#[test]
fn test_iter() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let _ = arena.test_alloc(1);
    let id2 = arena.test_alloc(2);
    let _ = arena.test_alloc(3);

    // Remove middle item
    arena.remove(id2);

    let items: Vec<_> = arena.iter().copied().collect();
    assert_eq!(items, vec![1, 3]);
}

#[test]
fn test_iter_mut() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    arena.test_alloc_extend([1, 2, 3]);

    for item in arena.iter_mut() {
        *item *= 2;
    }

    let items: Vec<_> = arena.iter().copied().collect();
    assert_eq!(items, vec![2, 4, 6]);
}

#[test]
fn test_drain() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    arena.test_alloc_extend([1, 2, 3]);

    let drained: Vec<_> = arena.drain().collect();
    assert_eq!(drained, vec![1, 2, 3]);
    assert_empty(&arena);
    assert_eq!(arena.free_count(), 3);
}

#[test]
fn test_retain() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    arena.test_alloc_extend([1, 2, 3, 4, 5]);

    arena.retain(|_, v| *v % 2 == 1);

    let items: Vec<_> = arena.iter().copied().collect();
    assert_eq!(items, vec![1, 3, 5]);
    assert_len(&arena, 3);
}

#[test]
fn test_clear() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.test_alloc(10);
    let _ = arena.test_alloc(20);

    arena.clear();

    assert_empty(&arena);
    assert!(arena.get(id1).is_none());
    assert_eq!(arena.free_count(), 2);
}

#[test]
fn test_generation_overflow() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let _id = arena.test_alloc(42);

    // NOTE: Direct field access is necessary to test generation overflow behavior.
    // There is no public API to set generation values for testing edge cases.
    arena.slots[0].generation = u32::MAX - 1;
    arena.free.push(0);
    arena.slots[0].value = None;
    arena.len = 0;

    // Allocate: generation should be MAX-1
    let id1 = arena.test_alloc(100);
    assert_eq!(id1.generation(), u32::MAX - 1);

    // Remove: generation should wrap to MAX
    arena.remove(id1);

    // Allocate again: generation should be MAX
    let id2 = arena.test_alloc(200);
    assert_eq!(id2.generation(), u32::MAX);

    // Remove: generation should wrap to 0
    arena.remove(id2);

    // Allocate again: generation should be 0
    let id3 = arena.test_alloc(300);
    assert_eq!(id3.generation(), 0);
}

#[test]
fn test_alloc_with() {
    struct Node {
        id: Id<Node>,
        value: i32,
    }
    let mut arena: SlotArena<Node> = SlotArena::new();
    let id = arena.test_alloc_with(|id| Node { id, value: 42 });
    assert_eq!(arena[id].id, id);
    assert_eq!(arena[id].value, 42);
}
