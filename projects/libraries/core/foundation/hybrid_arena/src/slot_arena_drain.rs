// projects/libraries/hybrid_arena/src/slot_arena_drain.rs
use crate::SlotArena;

/// A draining iterator for `SlotArena`.
#[derive(Debug)]
pub struct SlotArenaDrain<'a, T> {
    pub arena: &'a mut SlotArena<T>,
    pub index: usize,
}

impl<T> Iterator for SlotArenaDrain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.arena.slots.len() {
            let slot = &mut self.arena.slots[self.index];
            self.index += 1;
            if let Some(value) = slot.value.take() {
                slot.generation = slot.generation.wrapping_add(1);
                self.arena.free.push((self.index - 1) as u32);
                self.arena.len -= 1;
                return Some(value);
            }
        }
        None
    }
}

impl<T> Drop for SlotArenaDrain<'_, T> {
    fn drop(&mut self) {
        // Exhaust the iterator to ensure all items are drained
        while self.next().is_some() {}
    }
}
