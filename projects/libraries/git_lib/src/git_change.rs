// projects/libraries/git_lib/src/git_change.rs
use serde::{Deserialize, Serialize};

/// Represents a parsed Git change (porcelain -z).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitChange {
    pub xy: [u8; 2], // Example: [b'M', b' '] ; [b'R', b' ']
    pub path: String,
    pub orig_path: Option<String>, // For renames/copies
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitStatus(pub [u8; 2]);

impl GitStatus {
    #[inline]
    pub fn as_bytes(self) -> [u8; 2] {
        self.0
    }

    #[inline]
    pub fn as_str(self) -> &'static str {
        // Small fast mapping for common statuses; fallback is "??"
        match self.0 {
            [b' ', b'M'] => " M",
            [b'M', b' '] => "M ",
            [b'M', b'M'] => "MM",
            [b'A', b' '] => "A ",
            [b'D', b' '] => "D ",
            [b'R', b' '] => "R ",
            [b'C', b' '] => "C ",
            [b'?', b'?'] => "??",
            [b'U', b'U'] => "UU",
            [b'U', b' '] => "U ",
            [b' ', b'U'] => " U",
            _ => "??",
        }
    }

    #[inline]
    pub fn to_string_lossy(self) -> String {
        // exact 2 chars, safe and tiny
        let x = self.0[0] as char;
        let y = self.0[1] as char;
        format!("{x}{y}")
    }

    #[inline]
    pub fn is_rename_or_copy(self) -> bool {
        matches!(self.0[0], b'R' | b'C')
    }

    #[inline]
    pub fn is_unmerged(self) -> bool {
        let [x, y] = self.0;
        x == b'U' || y == b'U' || (x == b'A' && y == b'A') || (x == b'D' && y == b'D')
    }
}

impl GitChange {
    #[inline]
    pub fn status(&self) -> GitStatus {
        GitStatus(self.xy)
    }

    /// Allocates a 2-char string, use sparingly.
    #[inline]
    pub fn status_string(&self) -> String {
        self.status().to_string_lossy()
    }
}
