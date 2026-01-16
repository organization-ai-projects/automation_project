use crate::Slot;

/// An iterator over references to active items in a `SlotArena`.
#[derive(Debug)]
pub struct SlotArenaIter<'a, T> {
    pub slots: std::iter::Enumerate<std::slice::Iter<'a, Slot<T>>>,
    pub remaining: usize,
}

impl<'a, T> Iterator for SlotArenaIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        loop {
            let (_, slot) = self.slots.next()?;
            if let Some(ref value) = slot.value {
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

impl<T> ExactSizeIterator for SlotArenaIter<'_, T> {}
impl<T> std::iter::FusedIterator for SlotArenaIter<'_, T> {}
