// projects/libraries/hybrid_arena/src/bump_arena_into_iter.rs
/// An owning iterator over items in a `BumpArena`.
#[derive(Debug)]
pub struct BumpArenaIntoIter<T> {
    pub inner: std::vec::IntoIter<T>,
    pub index: u32,
}

impl<T> Iterator for BumpArenaIntoIter<T> {
    type Item = T;

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

impl<T> ExactSizeIterator for BumpArenaIntoIter<T> {}
impl<T> std::iter::FusedIterator for BumpArenaIntoIter<T> {}
