// projects/libraries/ast_core/src/validate_limits.rs
/// Validation limits for AST structures.
///
/// - `max_depth`: Maximum depth of the AST (root = 1).
/// - `max_size`: Maximum number of elements in arrays/objects.
#[derive(Clone, Debug)]
pub struct ValidateLimits {
    /// Maximum depth (root = 1).
    pub max_depth: usize,
    /// Maximum number of elements in arrays/objects.
    pub max_size: usize,
}

impl ValidateLimits {
    /// Strict limits for JSON-like structures.
    pub fn strict_json() -> Self {
        Self {
            max_depth: 32,    // Reasonable for JSON
            max_size: 10_000, // Prevent abuse
        }
    }

    /// No limits (use with caution).
    pub fn unbounded() -> Self {
        Self {
            max_depth: usize::MAX,
            max_size: usize::MAX,
        }
    }

    /// Validates if a float value is acceptable under strict JSON rules.
    pub fn validate_float(value: f64) -> Result<(), &'static str> {
        if value.is_nan() || value.is_infinite() {
            Err("Invalid float: NaN or Infinity is not allowed in strict JSON")
        } else {
            Ok(())
        }
    }
}

impl Default for ValidateLimits {
    fn default() -> Self {
        Self {
            max_depth: 256,
            max_size: 100_000,
        }
    }
}
