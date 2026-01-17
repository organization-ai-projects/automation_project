//! Bump arena for fast, append-only allocation.
//!
//! The bump arena is optimized for scenarios where you allocate many items
//! and never remove them individually. It provides O(1) allocation with
//! excellent cache locality.
// projects/libraries/hybrid_arena/src/bump_arena.rs
use std::ops::{Index, IndexMut};
use std::slice;

use crate::common_methods::{
    ArenaCommon, clear_vec, new_arena, reserve_capacity, with_capacity_arena,
};
use crate::error::ArenaError;
use crate::id::Id;
use crate::{BumpArenaDrain, BumpArenaIntoIter, BumpArenaIter, BumpArenaIterMut};

// Redéfinition directe du type IntoIter
type IntoIter<T> = BumpArenaIntoIter<T>;

#[derive(Debug)]
pub struct BumpArena<T> {
    pub items: Vec<T>,
}

impl<T> BumpArena<T> {
    /// Creates an empty arena.
    #[inline]
    pub fn new() -> Self {
        Self { items: new_arena() }
    }

    /// Creates an arena with pre-allocated capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: with_capacity_arena(capacity),
        }
    }

    /// Allocates an item and returns its ID.
    ///
    /// # Errors
    /// Returns `ArenaError::Overflow` if the arena contains more than 2^32 items.
    #[inline]
    pub fn alloc(&mut self, item: T) -> Result<Id<T>, ArenaError> {
        let index: u32 = self
            .items
            .len()
            .try_into()
            .map_err(|_| ArenaError::Overflow)?;
        self.items.push(item);
        Ok(Id::new(index, 0))
    }

    /// Allocates an item using a closure that receives the ID.
    ///
    /// Useful for self-referential structures.
    #[inline]
    pub fn alloc_with<F>(&mut self, f: F) -> Result<Id<T>, ArenaError>
    where
        F: FnOnce(Id<T>) -> T,
    {
        let index: u32 = self
            .items
            .len()
            .try_into()
            .map_err(|_| ArenaError::Overflow)?;
        let id = Id::new(index, 0);
        self.items.push(f(id));
        Ok(id)
    }

    /// Allocates multiple items from an iterator.
    ///
    /// Returns a vector of IDs in allocation order.
    #[inline]
    pub fn alloc_extend<I>(&mut self, iter: I) -> Result<Vec<Id<T>>, ArenaError>
    where
        I: IntoIterator<Item = T>,
    {
        <Self as ArenaCommon<T>>::alloc_extend_common(self, iter)
    }

    /// Returns a reference to the item with the given ID.
    #[inline]
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        <Self as ArenaCommon<T>>::get_common(self, id)
    }

    /// Returns a mutable reference to the item with the given ID.
    #[inline]
    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        <Self as ArenaCommon<T>>::get_mut_common(self, id)
    }

    /// Returns references to two items simultaneously.
    ///
    /// # Panics
    /// Panics if the two IDs refer to the same item.
    #[inline]
    pub fn get2_mut(&mut self, id1: Id<T>, id2: Id<T>) -> (Option<&mut T>, Option<&mut T>) {
        <Self as ArenaCommon<T>>::get2_mut_common(
            self,
            id1,
            id2,
            "cannot borrow the same item twice",
        )
    }

    /// Returns true if the arena contains the given ID.
    #[inline]
    pub fn contains(&self, id: Id<T>) -> bool {
        <Self as ArenaCommon<T>>::contains_common(self, id)
    }

    /// Returns the number of items in the arena.
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the arena is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the current capacity.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Reserves capacity for at least `additional` more items.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        reserve_capacity(&mut self.items, additional);
    }

    /// Shrinks the capacity to fit the current length.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
    }

    /// Clears the arena, removing all items.
    ///
    /// Note: This invalidates all existing IDs.
    #[inline]
    pub fn clear(&mut self) {
        clear_vec(&mut self.items);
    }

    /// Returns an iterator over references to all items.
    #[inline]
    pub fn iter(&self) -> BumpArenaIter<'_, T> {
        BumpArenaIter {
            inner: self.items.iter(),
            index: 0,
        }
    }

    /// Returns an iterator over mutable references to all items.
    #[inline]
    pub fn iter_mut(&mut self) -> BumpArenaIterMut<'_, T> {
        BumpArenaIterMut {
            inner: self.items.iter_mut(),
            index: 0,
        }
    }

    /// Returns a draining iterator that removes and yields all items.
    #[inline]
    pub fn drain(&mut self) -> BumpArenaDrain<'_, T> {
        BumpArenaDrain {
            inner: self.items.drain(..),
            index: 0,
        }
    }

    /// Returns an iterator over IDs only.
    #[inline]
    pub fn ids(&self) -> impl Iterator<Item = Id<T>> + '_ {
        (0..self.items.len() as u32).map(|i| Id::new(i, 0))
    }

    /// Returns an iterator over (ID, &T) pairs.
    #[inline]
    pub fn iter_with_ids(&self) -> impl Iterator<Item = (Id<T>, &T)> + '_ {
        self.items
            .iter()
            .enumerate()
            .map(|(i, item)| (Id::new(i as u32, 0), item))
    }

    /// Returns an iterator over (ID, &mut T) pairs.
    #[inline]
    pub fn iter_mut_with_ids(&mut self) -> impl Iterator<Item = (Id<T>, &mut T)> + '_ {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(i, item)| (Id::new(i as u32, 0), item))
    }

    /// Returns the underlying Vec, consuming the arena.
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }

    /// Creates an arena from a Vec.
    ///
    /// # Errors
    /// Returns `ArenaError::Overflow` if the Vec has more than 2^32 items.
    #[inline]
    pub fn from_vec(items: Vec<T>) -> Result<Self, ArenaError> {
        if items.len() > u32::MAX as usize {
            return Err(ArenaError::Overflow);
        }
        Ok(Self { items })
    }

    /// Returns the item without bounds or generation checking.
    ///
    /// # Safety
    /// The caller must ensure `id` is valid (index < len, generation == 0).
    #[inline]
    pub unsafe fn get_unchecked(&self, id: Id<T>) -> &T {
        debug_assert!(id.generation() == 0, "BumpArena only uses generation 0");
        debug_assert!(
            (id.index() as usize) < self.items.len(),
            "index out of bounds"
        );
        // SAFETY: The caller ensures the safety conditions.
        unsafe { self.items.get_unchecked(id.index() as usize) }
    }

    /// Returns a mutable reference without bounds or generation checking.
    ///
    /// # Safety
    /// The caller must ensure `id` is valid (index < len, generation == 0).
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, id: Id<T>) -> &mut T {
        debug_assert!(id.generation() == 0, "BumpArena only uses generation 0");
        debug_assert!(
            (id.index() as usize) < self.items.len(),
            "index out of bounds"
        );
        // SAFETY: The caller ensures the safety conditions.
        unsafe { self.items.get_unchecked_mut(id.index() as usize) }
    }

    /// Safe wrapper for `get_unchecked`.
    ///
    /// Cette méthode vérifie que l'ID est valide avant d'accéder à l'élément.
    #[inline]
    pub fn get_safe(&self, id: Id<T>) -> Option<&T> {
        if id.generation() != 0 || id.index() as usize >= self.items.len() {
            return None;
        }
        // SAFETY: Nous avons vérifié les limites et la génération.
        Some(unsafe { self.get_unchecked(id) })
    }

    /// Safe wrapper for `get_unchecked_mut`.
    ///
    /// Cette méthode vérifie que l'ID est valide avant d'accéder à l'élément mutable.
    #[inline]
    pub fn get_safe_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        if id.generation() != 0 || id.index() as usize >= self.items.len() {
            return None;
        }
        // SAFETY: Nous avons vérifié les limites et la génération.
        Some(unsafe { self.get_unchecked_mut(id) })
    }
}

impl<T> ArenaCommon<T> for BumpArena<T> {
    type Slot = T;

    #[inline]
    fn slots(&self) -> &[Self::Slot] {
        self.items.as_slice()
    }

    #[inline]
    fn slots_mut(&mut self) -> &mut [Self::Slot] {
        self.items.as_mut_slice()
    }

    #[inline]
    fn slot_value_ref(slot: &Self::Slot, id: Id<T>) -> Option<&T> {
        if id.generation() != 0 {
            return None;
        }
        Some(slot)
    }

    #[inline]
    fn slot_value_mut(slot: &mut Self::Slot, id: Id<T>) -> Option<&mut T> {
        if id.generation() != 0 {
            return None;
        }
        Some(slot)
    }

    #[inline]
    fn alloc(&mut self, item: T) -> Result<Id<T>, ArenaError> {
        BumpArena::alloc(self, item)
    }
}

impl<T> Default for BumpArena<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Id<T>> for BumpArena<T> {
    type Output = T;

    #[inline]
    fn index(&self, id: Id<T>) -> &Self::Output {
        assert!(id.generation() == 0, "invalid arena ID: wrong generation");
        assert!(
            (id.index() as usize) < self.items.len(),
            "invalid arena ID: index out of bounds"
        );
        // SAFETY: we just validated generation and bounds
        unsafe { self.get_unchecked(id) }
    }
}

impl<T> IndexMut<Id<T>> for BumpArena<T> {
    #[inline]
    fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
        assert!(id.generation() == 0, "invalid arena ID: wrong generation");
        assert!(
            (id.index() as usize) < self.items.len(),
            "invalid arena ID: index out of bounds"
        );
        // SAFETY: we just validated generation and bounds
        unsafe { self.get_unchecked_mut(id) }
    }
}

impl<T> FromIterator<T> for BumpArena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<T> = iter.into_iter().collect();
        Self { items }
    }
}

impl<T> Extend<T> for BumpArena<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}

impl<T> IntoIterator for BumpArena<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.items.into_iter(),
            index: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a BumpArena<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut BumpArena<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

// ============================================================================
// Serde support
// ============================================================================

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize> Serialize for BumpArena<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.items.serialize(serializer)
        }
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for BumpArena<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let items = Vec::<T>::deserialize(deserializer)?;
            Self::from_vec(items).map_err(serde::de::Error::custom)
        }
    }
}
