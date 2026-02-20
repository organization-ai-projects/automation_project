// projects/libraries/common_json/src/patch_op.rs
use crate::Json;

/// JSON patch operation (RFC 6902 style).
///
/// **Note**: This type is defined but the operations are not yet implemented.
#[derive(Debug, Clone)]
pub enum PatchOp {
    /// Adds a value to a path.
    Add {
        /// JSON Pointer path.
        path: String,
        /// Value to add.
        value: Json,
    },
    /// Removes the value at a path.
    Remove {
        /// JSON Pointer path.
        path: String,
    },
    /// Replaces the value at a path.
    Replace {
        /// JSON Pointer path.
        path: String,
        /// New value.
        value: Json,
    },
    /// Moves a value from one path to another.
    Move {
        /// Source path.
        from: String,
        /// Destination path.
        to: String,
    },
    /// Copies a value from one path to another.
    Copy {
        /// Source path.
        from: String,
        /// Destination path.
        to: String,
    },
}
