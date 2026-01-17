// projects/libraries/hybrid_arena/src/slot_arena_iter_mut.rs
use crate::Slot;

/// An iterator over mutable references to active items in a `SlotArena`.
#[derive(Debug)]
pub struct SlotArenaIterMut<'a, T> {
    pub slots: std::iter::Enumerate<std::slice::IterMut<'a, Slot<T>>>,
    pub remaining: usize,
}

impl<'a, T> Iterator for SlotArenaIterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        loop {
            let (_, slot) = self.slots.next()?;
            if let Some(ref mut value) = slot.value {
                self.remaining -= 1;
                return Some(value);
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> ExactSizeIterator for SlotArenaIterMut<'_, T> {}
impl<T> std::iter::FusedIterator for SlotArenaIterMut<'_, T> {}
