//! Typed ID for referencing items in arenas.
//!
//! The ID is packed into a single u64 for cache efficiency:
//! - High 32 bits: generation (for ABA problem prevention)
//! - Low 32 bits: index
// projects/libraries/hybrid_arena/src/id.rs
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// A typed, generation-checked ID for referencing items in arenas.
///
/// Packed into a single `u64` for optimal cache performance:
/// - Bits 0-31: index (max 4 billion items)
/// - Bits 32-63: generation (for use-after-free detection)
///
/// The type parameter `T` ensures compile-time safety: you can't use an
/// `Id<Foo>` to access a `BumpArena<Bar>`.
#[repr(transparent)]
pub struct Id<T> {
    pub packed: u64,
    pub marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    /// Maximum valid index (2^32 - 1).
    pub const MAX_INDEX: u32 = u32::MAX;

    /// Maximum valid generation (2^32 - 1).
    pub const MAX_GENERATION: u32 = u32::MAX;

    /// Creates a new ID from index and generation.
    #[inline(always)]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self {
            packed: (generation as u64) << 32 | (index as u64),
            marker: PhantomData,
        }
    }

    /// Creates an ID from a raw packed u64.
    #[inline(always)]
    pub const fn from_raw(packed: u64) -> Self {
        Self {
            packed,
            marker: PhantomData,
        }
    }

    /// Returns the raw packed u64.
    #[inline(always)]
    pub const fn to_raw(self) -> u64 {
        self.packed
    }

    /// Returns the index component.
    #[inline(always)]
    pub const fn index(self) -> u32 {
        self.packed as u32
    }

    /// Returns the generation component.
    #[inline(always)]
    pub const fn generation(self) -> u32 {
        (self.packed >> 32) as u32
    }

    /// Creates a new ID with an incremented generation (wrapping).
    #[inline(always)]
    pub const fn next_generation(self) -> Self {
        Self::new(self.index(), self.generation().wrapping_add(1))
    }

    /// Casts this ID to a different type. Use with extreme caution.
    ///
    /// # Safety
    /// The caller must ensure that the arena this ID refers to
    /// actually contains items of type `U`.
    #[inline(always)]
    pub const unsafe fn cast<U>(self) -> Id<U> {
        Id {
            packed: self.packed,
            marker: PhantomData,
        }
    }
}

// Manual impls to avoid requiring T: Trait bounds

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for Id<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.packed == other.packed
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.packed.hash(state);
    }
}

impl<T> PartialOrd for Id<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.packed.cmp(&other.packed)
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Id")
            .field("index", &self.index())
            .field("generation", &self.generation())
            .finish()
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.index(), self.generation())
    }
}

impl<T> Default for Id<T> {
    /// Returns an ID with index 0 and generation 0.
    #[inline(always)]
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T> Serialize for Id<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.packed.serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Id<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            Ok(Self::from_raw(u64::deserialize(deserializer)?))
        }
    }
}
