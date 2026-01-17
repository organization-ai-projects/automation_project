// projects/libraries/hybrid_arena/src/bump_arena_iter.rs
use std::slice;

/// An iterator over references to items in a `BumpArena`.
#[derive(Debug)]
pub struct BumpArenaIter<'a, T> {
    pub inner: slice::Iter<'a, T>,
    pub index: u32,
}

impl<'a, T> Iterator for BumpArenaIter<'a, T> {
    type Item = &'a T;

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

impl<T> ExactSizeIterator for BumpArenaIter<'_, T> {}
impl<T> std::iter::FusedIterator for BumpArenaIter<'_, T> {}
