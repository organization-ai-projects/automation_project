// projects/libraries/hybrid_arena/src/tests/slot_arena_test.rs
use crate::{Id, SlotArena};

#[test]
fn test_alloc_and_get() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.alloc(10).unwrap();
    let id2 = arena.alloc(20).unwrap();

    assert_eq!(arena.get(id1), Some(&10));
    assert_eq!(arena.get(id2), Some(&20));
    assert_eq!(arena.len(), 2);
}

#[test]
fn test_remove_and_reuse() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.alloc(10).unwrap();
    let id2 = arena.alloc(20).unwrap();

    // Remove first item
    assert_eq!(arena.remove(id1), Some(10));
    assert!(arena.get(id1).is_none());
    assert_eq!(arena.len(), 1);

    // Allocate new item - should reuse slot 0
    let id3 = arena.alloc(30).unwrap();
    assert_eq!(id3.index(), id1.index());
    assert_ne!(id3.generation(), id1.generation()); // Generation bumped
    assert_eq!(arena[id3], 30);
    assert_eq!(arena.len(), 2);

    // Old ID should still be invalid
    assert!(arena.get(id1).is_none());
    assert_eq!(arena.get(id2), Some(&20));
}

#[test]
fn test_generation_prevents_use_after_free() {
    let mut arena: SlotArena<String> = SlotArena::new();
    let id = arena.alloc("first".to_string()).unwrap();
    let old_gen = id.generation();

    arena.remove(id);
    let new_id = arena.alloc("second".to_string()).unwrap();

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
    let id1 = arena.alloc(10).unwrap();
    let id2 = arena.alloc(20).unwrap();

    let (a, b) = arena.get_mut(id1, id2);
    *a.unwrap() += 1;
    *b.unwrap() += 1;

    assert_eq!(arena[id1], 11);
    assert_eq!(arena[id2], 21);
}

#[test]
#[should_panic(expected = "cannot borrow the same slot twice")]
fn test_get2_mut_same_id_panics() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id = arena.alloc(10).unwrap();
    let _ = arena.get_mut(id, id);
}

#[test]
fn test_iter() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let _ = arena.alloc(1).unwrap();
    let id2 = arena.alloc(2).unwrap();
    let _ = arena.alloc(3).unwrap();

    // Remove middle item
    arena.remove(id2);

    let items: Vec<_> = arena.iter().copied().collect();
    assert_eq!(items, vec![1, 3]);
}

#[test]
fn test_iter_mut() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();

    for item in arena.iter_mut() {
        *item *= 2;
    }

    let items: Vec<_> = arena.iter().copied().collect();
    assert_eq!(items, vec![2, 4, 6]);
}

#[test]
fn test_drain() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();

    let drained: Vec<_> = arena.drain().collect();
    assert_eq!(drained, vec![1, 2, 3]);
    assert!(arena.is_empty());
    assert_eq!(arena.free_count(), 3);
}

#[test]
fn test_retain() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    arena.alloc_extend([1, 2, 3, 4, 5]).unwrap();

    arena.retain(|_, v| *v % 2 == 1);

    let items: Vec<_> = arena.iter().copied().collect();
    assert_eq!(items, vec![1, 3, 5]);
    assert_eq!(arena.len(), 3);
}

#[test]
fn test_clear() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let id1 = arena.alloc(10).unwrap();
    let _ = arena.alloc(20).unwrap();

    arena.clear();

    assert!(arena.is_empty());
    assert!(arena.get(id1).is_none());
    assert_eq!(arena.free_count(), 2);
}

#[test]
fn test_generation_overflow() {
    let mut arena: SlotArena<i32> = SlotArena::new();
    let _id = arena.alloc(42).unwrap();

    // Manually set generation to MAX-1 to test wrapping
    arena.slots[0].generation = u32::MAX - 1;
    arena.free.push(0);
    arena.slots[0].value = None;
    arena.len = 0;

    // Allocate: generation should be MAX-1
    let id1 = arena.alloc(100).unwrap();
    assert_eq!(id1.generation(), u32::MAX - 1);

    // Remove: generation should wrap to MAX
    arena.remove(id1);

    // Allocate again: generation should be MAX
    let id2 = arena.alloc(200).unwrap();
    assert_eq!(id2.generation(), u32::MAX);

    // Remove: generation should wrap to 0
    arena.remove(id2);

    // Allocate again: generation should be 0
    let id3 = arena.alloc(300).unwrap();
    assert_eq!(id3.generation(), 0);
}

#[test]
fn test_alloc_with() {
    struct Node {
        id: Id<Node>,
        value: i32,
    }
    let mut arena: SlotArena<Node> = SlotArena::new();
    let id = arena.alloc_with(|id| Node { id, value: 42 }).unwrap();
    assert_eq!(arena[id].id, id);
    assert_eq!(arena[id].value, 42);
}
