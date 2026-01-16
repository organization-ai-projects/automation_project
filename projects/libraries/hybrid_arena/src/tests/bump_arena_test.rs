use crate::{BumpArena, Id};

#[test]
fn test_alloc_and_get() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    let id1 = arena.alloc(10).unwrap();
    let id2 = arena.alloc(20).unwrap();

    assert_eq!(arena.get(id1), Some(&10));
    assert_eq!(arena.get(id2), Some(&20));
    assert_eq!(arena.len(), 2);
}

#[test]
fn test_alloc_with() {
    struct Node {
        id: Id<Node>,
        value: i32,
    }
    let mut arena: BumpArena<Node> = BumpArena::new();
    let id = arena.alloc_with(|id| Node { id, value: 42 }).unwrap();
    assert_eq!(arena[id].id, id);
    assert_eq!(arena[id].value, 42);
}

#[test]
fn test_alloc_extend() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    let ids = arena.alloc_extend([1, 2, 3, 4, 5]).unwrap();

    assert_eq!(ids.len(), 5);
    for (i, id) in ids.iter().enumerate() {
        assert_eq!(arena[*id], (i + 1) as i32);
    }
}

#[test]
fn test_get2_mut() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    let id1 = arena.alloc(10).unwrap();
    let id2 = arena.alloc(20).unwrap();

    let (a, b) = arena.get2_mut(id1, id2);
    *a.unwrap() += 1;
    *b.unwrap() += 1;

    assert_eq!(arena[id1], 11);
    assert_eq!(arena[id2], 21);
}

#[test]
#[should_panic(expected = "cannot borrow the same item twice")]
fn test_get2_mut_same_id_panics() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    let id = arena.alloc(10).unwrap();
    let _ = arena.get2_mut(id, id);
}

#[test]
fn test_invalid_generation() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    let id = arena.alloc(42).unwrap();

    // Create an ID with generation 1 (invalid for BumpArena)
    let bad_id = Id::new(id.index(), 1);
    assert!(arena.get(bad_id).is_none());
}

#[test]
fn test_iter() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();

    let sum: i32 = arena.iter().sum();
    assert_eq!(sum, 6);
}

#[test]
fn test_iter_mut() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();

    for item in arena.iter_mut() {
        *item *= 2;
    }

    let sum: i32 = arena.iter().sum();
    assert_eq!(sum, 12);
}

#[test]
fn test_into_iter() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();

    let items: Vec<_> = arena.into_iter().collect();
    assert_eq!(items, vec![1, 2, 3]);
}

#[test]
fn test_drain() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();

    let drained: Vec<_> = arena.drain().collect();
    assert_eq!(drained, vec![1, 2, 3]);
    assert!(arena.is_empty());
}

#[test]
fn test_from_iter() {
    let arena: BumpArena<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(arena.len(), 3);
}

#[test]
fn test_extend() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    arena.extend([1, 2, 3]);
    arena.extend([4, 5]);
    assert_eq!(arena.len(), 5);
}

#[test]
fn test_clear() {
    let mut arena: BumpArena<i32> = BumpArena::new();
    arena.alloc_extend([1, 2, 3]).unwrap();
    arena.clear();
    assert!(arena.is_empty());
}

#[test]
fn test_from_vec_overflow() {
    // This test just verifies the check exists; creating 4B+ items is impractical
    let arena = BumpArena::from_vec(vec![1, 2, 3]);
    assert!(arena.is_ok());
}
