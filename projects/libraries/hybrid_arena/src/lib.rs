//! # `hybrid_arena` - High-Performance Arena Allocators
// projects/libraries/hybrid_arena/src/lib.rs
mod arena_common_trait;
pub mod bump_arena;
pub mod bump_arena_drain;
pub mod bump_arena_into_iter;
pub mod bump_arena_iter;
pub mod bump_arena_iter_mut;
pub mod common_methods;
pub mod error;
pub mod id;
pub mod slot;
pub mod slot_arena;
pub mod slot_arena_drain;
pub mod slot_arena_iter;
pub mod slot_arena_iter_mut;

// Re-export main types
pub use bump_arena::BumpArena;
pub use bump_arena_drain::BumpArenaDrain;
pub use bump_arena_into_iter::BumpArenaIntoIter;
pub use bump_arena_iter::BumpArenaIter;
pub use bump_arena_iter_mut::BumpArenaIterMut;
pub use common_methods::{
    clear_vec, is_valid_id, new_arena, reserve_capacity, with_capacity_arena,
};
pub use error::ArenaError;
pub use id::Id;
pub use slot::Slot;
pub use slot_arena::SlotArena;
pub use slot_arena_drain::SlotArenaDrain;
pub use slot_arena_iter::SlotArenaIter;
pub use slot_arena_iter_mut::SlotArenaIterMut;

// Re-export iterator types for advanced use

/// Prelude module for convenient imports.
///
/// ```rust
/// use hybrid_arena::prelude::*;
/// ```
pub mod prelude {
    pub use crate::bump_arena::BumpArena;
    pub use crate::error::ArenaError;
    pub use crate::id::Id;
    pub use crate::slot_arena::SlotArena;
}

#[cfg(test)]
mod tests;
