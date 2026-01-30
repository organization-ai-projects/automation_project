//! Common module for shared methods between SlotArena and BumpArena.
// projects/libraries/hybrid_arena/src/common_methods.rs
use crate::id::Id;

/// Common function to check if an ID is valid.
#[inline]
pub fn is_valid_id<T>(id: Id<T>, len: usize, generation: u32) -> bool {
    (id.index() as usize) < len && id.generation() == generation
}

/// Common function to reserve capacity.
#[inline]
pub fn reserve_capacity<T>(vec: &mut Vec<T>, additional: usize) {
    vec.reserve(additional);
}

/// Common function to clear a vector.
#[inline]
pub fn clear_vec<T>(vec: &mut Vec<T>) {
    vec.iter_mut().for_each(|_| {});
    vec.clear();
}

/// Common function to create a new empty arena.
#[inline]
pub fn new_arena<T>() -> Vec<T> {
    Vec::new()
}

/// Common function to create an arena with pre-allocated capacity.
#[inline]
pub fn with_capacity_arena<T>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(capacity)
}
