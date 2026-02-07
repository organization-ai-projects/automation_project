// projects/libraries/hybrid_arena/src/tests/helpers.rs
//! Shared test helpers to reduce duplication across arena tests.

use crate::Id;

/// Helper trait for common arena test operations.
pub trait ArenaTestHelpers<T> {
    /// Allocate an item and unwrap the result.
    fn test_alloc(&mut self, item: T) -> Id<T>;

    /// Allocate with a closure and unwrap the result.
    fn test_alloc_with<F>(&mut self, f: F) -> Id<T>
    where
        F: FnOnce(Id<T>) -> T;

    /// Allocate multiple items and unwrap the result.
    fn test_alloc_extend<I>(&mut self, iter: I) -> Vec<Id<T>>
    where
        I: IntoIterator<Item = T>;
}

/// Implement the helper trait for BumpArena.
impl<T> ArenaTestHelpers<T> for crate::BumpArena<T> {
    fn test_alloc(&mut self, item: T) -> Id<T> {
        self.alloc(item).expect("alloc should succeed")
    }

    fn test_alloc_with<F>(&mut self, f: F) -> Id<T>
    where
        F: FnOnce(Id<T>) -> T,
    {
        self.alloc_with(f).expect("alloc_with should succeed")
    }

    fn test_alloc_extend<I>(&mut self, iter: I) -> Vec<Id<T>>
    where
        I: IntoIterator<Item = T>,
    {
        self.alloc_extend(iter)
            .expect("alloc_extend should succeed")
    }
}

/// Implement the helper trait for SlotArena.
impl<T> ArenaTestHelpers<T> for crate::SlotArena<T> {
    fn test_alloc(&mut self, item: T) -> Id<T> {
        self.alloc(item).expect("alloc should succeed")
    }

    fn test_alloc_with<F>(&mut self, f: F) -> Id<T>
    where
        F: FnOnce(Id<T>) -> T,
    {
        self.alloc_with(f).expect("alloc_with should succeed")
    }

    fn test_alloc_extend<I>(&mut self, iter: I) -> Vec<Id<T>>
    where
        I: IntoIterator<Item = T>,
    {
        self.alloc_extend(iter)
            .expect("alloc_extend should succeed")
    }
}

/// Common assertion: verify arena length.
pub fn assert_len<A>(arena: &A, expected: usize)
where
    A: HasLen,
{
    assert_eq!(arena.len(), expected, "arena length mismatch");
}

/// Common assertion: verify arena is empty.
pub fn assert_empty<A>(arena: &A)
where
    A: HasLen,
{
    assert!(arena.is_empty(), "arena should be empty");
}

/// Trait for types that have len() and is_empty() methods.
///
/// This trait enables generic assertion helpers like `assert_len` and `assert_empty`
/// to work with both `BumpArena` and `SlotArena` without duplicating the assertion logic.
pub trait HasLen {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<T> HasLen for crate::BumpArena<T> {
    fn len(&self) -> usize {
        <crate::BumpArena<T>>::len(self)
    }
    fn is_empty(&self) -> bool {
        <crate::BumpArena<T>>::is_empty(self)
    }
}

impl<T> HasLen for crate::SlotArena<T> {
    fn len(&self) -> usize {
        <crate::SlotArena<T>>::len(self)
    }
    fn is_empty(&self) -> bool {
        <crate::SlotArena<T>>::is_empty(self)
    }
}
