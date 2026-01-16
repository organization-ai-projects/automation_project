//! Module commun pour les méthodes partagées entre SlotArena et BumpArena.

use crate::error::ArenaError;
use crate::id::Id;

/// Fonction commune pour vérifier si un ID est valide.
#[inline]
pub fn is_valid_id<T>(id: Id<T>, len: usize, generation: u32) -> bool {
    (id.index() as usize) < len && id.generation() == generation
}

/// Fonction commune pour réserver de la capacité.
#[inline]
pub fn reserve_capacity<T>(vec: &mut Vec<T>, additional: usize) {
    vec.reserve(additional);
}

/// Fonction commune pour vider un vecteur.
#[inline]
pub fn clear_vec<T>(vec: &mut Vec<T>) {
    vec.iter_mut().for_each(|_| {});
    vec.clear();
}

/// Fonction commune pour créer une nouvelle arène vide.
#[inline]
pub fn new_arena<T>() -> Vec<T> {
    Vec::new()
}

/// Fonction commune pour créer une arène avec une capacité pré-allouée.
#[inline]
pub fn with_capacity_arena<T>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(capacity)
}

/// Internal trait for sharing common arena logic.
pub(crate) trait ArenaCommon<T> {
    type Slot;

    fn slots(&self) -> &[Self::Slot];
    fn slots_mut(&mut self) -> &mut [Self::Slot];
    fn slot_value_ref(slot: &Self::Slot, id: Id<T>) -> Option<&T>;
    fn slot_value_mut(slot: &mut Self::Slot, id: Id<T>) -> Option<&mut T>;
    fn alloc(&mut self, item: T) -> Result<Id<T>, ArenaError>;

    #[inline]
    fn get_common(&self, id: Id<T>) -> Option<&T> {
        let idx = id.index() as usize;
        let slot = self.slots().get(idx)?;
        Self::slot_value_ref(slot, id)
    }

    #[inline]
    fn get_mut_common(&mut self, id: Id<T>) -> Option<&mut T> {
        let idx = id.index() as usize;
        let slot = self.slots_mut().get_mut(idx)?;
        Self::slot_value_mut(slot, id)
    }

    #[inline]
    fn contains_common(&self, id: Id<T>) -> bool {
        self.get_common(id).is_some()
    }

    #[inline]
    fn alloc_extend_common<I>(&mut self, iter: I) -> Result<Vec<Id<T>>, ArenaError>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        let mut ids = Vec::with_capacity(lower);

        for item in iter {
            ids.push(self.alloc(item)?);
        }

        Ok(ids)
    }

    #[inline]
    fn get2_mut_common(
        &mut self,
        id1: Id<T>,
        id2: Id<T>,
        duplicate_msg: &'static str,
    ) -> (Option<&mut T>, Option<&mut T>) {
        assert_ne!(id1.index(), id2.index(), "{}", duplicate_msg);

        let slots = self.slots_mut();
        let len = slots.len();
        let ptr = slots.as_mut_ptr();

        let get = |id: Id<T>| -> Option<&mut T> {
            let idx = id.index() as usize;
            if idx >= len {
                return None;
            }
            // SAFETY: indices are bounds-checked and id1 != id2.
            let slot = unsafe { &mut *ptr.add(idx) };
            Self::slot_value_mut(slot, id)
        };

        (get(id1), get(id2))
    }
}
