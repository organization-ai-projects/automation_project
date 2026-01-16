//! Slot arena with allocation, removal, and slot reuse.
//!
//! The slot arena supports dynamic allocation and deallocation with generation
//! tracking to prevent use-after-free bugs. Removed slots are recycled for
//! future allocations.

use std::ops::{Index, IndexMut};

use crate::common_methods::{
    ArenaCommon, clear_vec, new_arena, reserve_capacity, with_capacity_arena,
};
use crate::error::ArenaError;
use crate::id::Id;
use crate::{Slot, SlotArenaDrain, SlotArenaIter, SlotArenaIterMut};

/// A slot arena that supports allocation, removal, and generation-checked access.
///
/// # Performance characteristics
/// - Allocation: O(1)
/// - Removal: O(1)
/// - Access by ID: O(1)
/// - Iteration: O(capacity) (skips empty slots)
///
/// # Generation tracking
/// Each slot has a generation counter that increments on removal. IDs store
/// the generation at allocation time. Access with a stale ID (wrong generation)
/// returns `None` instead of accessing potentially reused memory.
///
/// # When to use
/// - Object pools where items are frequently added/removed
/// - Entity-Component-System (ECS) entity storage
/// - Handle-based resource management
/// - Anything requiring stable IDs that survive reallocation
///
/// # Example
/// ```
/// use hybrid_arena::{SlotArena, Id};
///
/// let mut arena: SlotArena<String> = SlotArena::new();
/// let id = arena.alloc("hello".to_string()).unwrap();
/// assert_eq!(arena.get(id), Some(&"hello".to_string()));
///
/// let removed = arena.remove(id);
/// assert_eq!(removed, Some("hello".to_string()));
/// assert!(arena.get(id).is_none()); // ID is now invalid
/// ```
#[derive(Debug)]
pub struct SlotArena<T> {
    pub slots: Vec<Slot<T>>,
    pub free: Vec<u32>,
    pub len: usize,
}

impl<T> SlotArena<T> {
    /// Creates an empty arena.
    #[inline]
    pub fn new() -> Self {
        Self {
            slots: new_arena(),
            free: new_arena(),
            len: 0,
        }
    }

    /// Creates an arena with pre-allocated capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: with_capacity_arena(capacity),
            free: new_arena(),
            len: 0,
        }
    }

    /// Fonction interne pour gérer l'allocation commune.
    #[inline]
    fn alloc_internal<F>(&mut self, initializer: F) -> Result<Id<T>, ArenaError>
    where
        F: FnOnce(Id<T>) -> T,
    {
        self.len += 1;

        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            debug_assert!(slot.value.is_none());
            let id = Id::new(index, slot.generation);
            slot.value = Some(initializer(id));
            Ok(id)
        } else {
            let index: u32 = self
                .slots
                .len()
                .try_into()
                .map_err(|_| ArenaError::Overflow)?;
            let id = Id::new(index, 0);
            self.slots.push(Slot {
                generation: 0,
                value: Some(initializer(id)),
            });
            Ok(id)
        }
    }

    /// Allocates an item and returns its ID.
    ///
    /// Reuses freed slots when available (LIFO order for cache friendliness).
    ///
    /// # Errors
    /// Returns `ArenaError::Overflow` if the arena would exceed 2^32 slots.
    #[inline]
    pub fn alloc(&mut self, item: T) -> Result<Id<T>, ArenaError> {
        self.alloc_internal(|_| item)
    }

    /// Allocates an item using a closure that receives the ID.
    ///
    /// Useful for self-referential structures.
    #[inline]
    pub fn alloc_with<F>(&mut self, f: F) -> Result<Id<T>, ArenaError>
    where
        F: FnOnce(Id<T>) -> T,
    {
        self.alloc_internal(f)
    }

    /// Allocates multiple items from an iterator.
    #[inline]
    pub fn alloc_extend<I>(&mut self, iter: I) -> Result<Vec<Id<T>>, ArenaError>
    where
        I: IntoIterator<Item = T>,
    {
        <Self as ArenaCommon<T>>::alloc_extend_common(self, iter)
    }

    /// Returns a reference to the item with the given ID.
    ///
    /// Returns `None` if the ID is invalid or the slot has been freed.
    #[inline]
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        <Self as ArenaCommon<T>>::get_common(self, id)
    }

    /// Returns mutable references to two items simultaneously.
    ///
    /// # Panics
    /// Panics if the two IDs refer to the same slot.
    #[inline]
    pub fn get_mut(&mut self, id1: Id<T>, id2: Id<T>) -> (Option<&mut T>, Option<&mut T>) {
        <Self as ArenaCommon<T>>::get2_mut_common(
            self,
            id1,
            id2,
            "cannot borrow the same slot twice",
        )
    }

    /// Removes and returns the item with the given ID.
    ///
    /// The slot is recycled for future allocations. The generation is
    /// incremented to invalidate any remaining references to this ID.
    #[inline]
    pub fn remove(&mut self, id: Id<T>) -> Option<T> {
        let slot = self.slots.get_mut(id.index() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }

        let value = slot.value.take()?;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(id.index());
        self.len -= 1;
        Some(value)
    }

    /// Removes an item without returning it (slightly faster).
    #[inline]
    pub fn remove_drop(&mut self, id: Id<T>) -> bool {
        self.remove(id).is_some()
    }

    /// Returns true if the ID refers to a valid item.
    #[inline]
    pub fn contains(&self, id: Id<T>) -> bool {
        <Self as ArenaCommon<T>>::contains_common(self, id)
    }

    /// Returns the number of active items.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the arena has no active items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the total slot capacity (including free slots).
    #[inline]
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Returns the number of allocated slots (including free slots).
    #[inline]
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    /// Returns the number of free slots available for reuse.
    #[inline]
    pub fn free_count(&self) -> usize {
        self.free.len()
    }

    /// Reserves capacity for at least `additional` more items.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        reserve_capacity(&mut self.slots, additional);
    }

    /// Clears the arena, removing all items.
    ///
    /// This resets all slots and invalidates all IDs. Generations are preserved
    /// so that old IDs remain invalid.
    #[inline]
    pub fn clear(&mut self) {
        clear_vec(&mut self.slots);
        clear_vec(&mut self.free);
        self.len = 0;
    }

    /// Removes all items but keeps the generation counters.
    /// More efficient than calling remove() on each item.
    #[inline]
    pub fn drain(&mut self) -> SlotArenaDrain<'_, T> {
        SlotArenaDrain {
            arena: self,
            index: 0,
        }
    }

    /// Returns an iterator over references to active items.
    #[inline]
    pub fn iter(&self) -> SlotArenaIter<'_, T> {
        SlotArenaIter {
            slots: self.slots.iter().enumerate(),
            remaining: self.len,
        }
    }

    /// Returns an iterator over mutable references to active items.
    #[inline]
    pub fn iter_mut(&mut self) -> SlotArenaIterMut<'_, T> {
        let remaining = self.len;
        SlotArenaIterMut {
            slots: self.slots.iter_mut().enumerate(),
            remaining,
        }
    }

    /// Returns an iterator over (ID, &T) pairs for active items.
    #[inline]
    pub fn iter_with_ids(&self) -> impl Iterator<Item = (Id<T>, &T)> + '_ {
        self.slots.iter().enumerate().filter_map(|(i, slot)| {
            slot.value
                .as_ref()
                .map(|v| (Id::new(i as u32, slot.generation), v))
        })
    }

    /// Returns an iterator over (ID, &mut T) pairs for active items.
    #[inline]
    pub fn iter_mut_with_ids(&mut self) -> impl Iterator<Item = (Id<T>, &mut T)> + '_ {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| {
            let generation = slot.generation;
            slot.value
                .as_mut()
                .map(|v| (Id::new(i as u32, generation), v))
        })
    }

    /// Returns an iterator over IDs of active items.
    #[inline]
    pub fn ids(&self) -> impl Iterator<Item = Id<T>> + '_ {
        self.slots.iter().enumerate().filter_map(|(i, slot)| {
            if slot.value.is_some() {
                Some(Id::new(i as u32, slot.generation))
            } else {
                None
            }
        })
    }

    /// Retains only the items for which the predicate returns true.
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(Id<T>, &mut T) -> bool,
    {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if let Some(ref mut value) = slot.value {
                let id = Id::new(i as u32, slot.generation);
                if !f(id, value) {
                    slot.value = None;
                    slot.generation = slot.generation.wrapping_add(1);
                    self.free.push(i as u32);
                    self.len -= 1;
                }
            }
        }
    }

    /// Returns the item without checking generation or bounds.
    ///
    /// # Safety
    /// The caller must ensure the ID is valid and the slot is occupied.
    #[inline]
    pub(crate) unsafe fn get_unchecked_id(&self, id: Id<T>) -> &T {
        assert!(
            (id.index() as usize) < self.slots.len(),
            "index out of bounds"
        );
        unsafe {
            let slot = self.slots.get_unchecked(id.index() as usize);
            assert!(slot.value.is_some(), "slot is empty");
            assert!(slot.generation == id.generation(), "generation mismatch");
            slot.value.as_ref().unwrap_unchecked()
        }
    }

    /// Returns a mutable reference without checking generation or bounds.
    ///
    /// # Safety
    /// The caller must ensure the ID is valid and the slot is occupied.
    #[inline]
    pub(crate) unsafe fn get_unchecked_id_mut(&mut self, id: Id<T>) -> &mut T {
        assert!(
            (id.index() as usize) < self.slots.len(),
            "index out of bounds"
        );
        unsafe {
            let slot = self.slots.get_unchecked_mut(id.index() as usize);
            assert!(slot.value.is_some(), "slot is empty");
            assert!(slot.generation == id.generation(), "generation mismatch");
            slot.value.as_mut().unwrap_unchecked()
        }
    }
}

impl<T> ArenaCommon<T> for SlotArena<T> {
    type Slot = Slot<T>;

    #[inline]
    fn slots(&self) -> &[Self::Slot] {
        self.slots.as_slice()
    }

    #[inline]
    fn slots_mut(&mut self) -> &mut [Self::Slot] {
        self.slots.as_mut_slice()
    }

    #[inline]
    fn slot_value_ref(slot: &Self::Slot, id: Id<T>) -> Option<&T> {
        if slot.generation != id.generation() {
            return None;
        }
        slot.value.as_ref()
    }

    #[inline]
    fn slot_value_mut(slot: &mut Self::Slot, id: Id<T>) -> Option<&mut T> {
        if slot.generation != id.generation() {
            return None;
        }
        slot.value.as_mut()
    }

    #[inline]
    fn alloc(&mut self, item: T) -> Result<Id<T>, ArenaError> {
        SlotArena::alloc(self, item)
    }
}

impl<T> Default for SlotArena<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Id<T>> for SlotArena<T> {
    type Output = T;

    #[inline]
    fn index(&self, id: Id<T>) -> &Self::Output {
        let idx = id.index() as usize;
        assert!(
            idx < self.slots.len(),
            "invalid arena ID: index out of bounds"
        );
        let slot = &self.slots[idx];
        assert!(
            slot.generation == id.generation(),
            "invalid arena ID: generation mismatch"
        );
        assert!(slot.value.is_some(), "invalid arena ID: slot is empty");
        // SAFETY: we just validated bounds, generation, and occupancy
        unsafe { self.get_unchecked_id(id) }
    }
}

impl<T> IndexMut<Id<T>> for SlotArena<T> {
    #[inline]
    fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
        let idx = id.index() as usize;
        assert!(
            idx < self.slots.len(),
            "invalid arena ID: index out of bounds"
        );
        assert!(
            self.slots[idx].generation == id.generation(),
            "invalid arena ID: generation mismatch"
        );
        assert!(
            self.slots[idx].value.is_some(),
            "invalid arena ID: slot is empty"
        );
        // SAFETY: we just validated bounds, generation, and occupancy
        unsafe { self.get_unchecked_id_mut(id) }
    }
}

impl<T> Extend<T> for SlotArena<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            let _ = self.alloc(item);
        }
    }
}

// ============================================================================
// Serde support
// ============================================================================

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize> Serialize for SlotArena<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut seq = serializer.serialize_seq(Some(self.len))?;
            for item in self.iter() {
                seq.serialize_element(item)?;
            }
            seq.end()
        }
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for SlotArena<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let items = Vec::<T>::deserialize(deserializer)?;
            let mut arena = SlotArena::with_capacity(items.len());
            for item in items {
                arena.alloc(item).map_err(serde::de::Error::custom)?;
            }
            Ok(arena)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
