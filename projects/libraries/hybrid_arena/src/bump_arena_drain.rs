//  projects/libraries/hybrid_arena/src/bump_arena_drain.rs
/// A draining iterator for `BumpArena`.
#[derive(Debug)]
pub struct BumpArenaDrain<'a, T> {
    pub inner: std::vec::Drain<'a, T>,
    pub index: u32,
}

impl<T> Iterator for BumpArenaDrain<'_, T> {
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

impl<T> ExactSizeIterator for BumpArenaDrain<'_, T> {}
impl<T> std::iter::FusedIterator for BumpArenaDrain<'_, T> {}
