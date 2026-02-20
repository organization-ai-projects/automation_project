// projects/libraries/hybrid_arena/src/bump_arena_iter_mut.rs
use std::slice;

/// An iterator over mutable references to items in a `BumpArena`.
#[derive(Debug)]
pub struct BumpArenaIterMut<'a, T> {
    pub inner: slice::IterMut<'a, T>,
    pub index: u32,
}

impl<'a, T> Iterator for BumpArenaIterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        self.index += 1;
        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> ExactSizeIterator for BumpArenaIterMut<'_, T> {}
impl<T> std::iter::FusedIterator for BumpArenaIterMut<'_, T> {}
