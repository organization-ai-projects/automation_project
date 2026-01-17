// projects/libraries/ast_core/src/number.rs
/// A numeric value preserving type information.
#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Int(i64),
    Uint(u64),
    Float(f64),
}

impl Number {
    /// Returns the value as i64 if it fits.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Int(n) => Some(*n),
            Number::Uint(n) if *n <= i64::MAX as u64 => Some(*n as i64),
            Number::Float(f)
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 =>
            {
                Some(*f as i64)
            }
            _ => None,
        }
    }

    /// Returns the value as u64 if it fits.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Number::Uint(n) => Some(*n),
            Number::Int(n) if *n >= 0 => Some(*n as u64),
            Number::Float(f) if f.fract() == 0.0 && *f >= 0.0 && *f <= u64::MAX as f64 => {
                Some(*f as u64)
            }
            _ => None,
        }
    }

    /// Returns the value as f64.
    pub fn as_f64(&self) -> f64 {
        match self {
            Number::Int(n) => *n as f64,
            Number::Uint(n) => *n as f64,
            Number::Float(f) => *f,
        }
    }

    /// Returns true if this is an integer type.
    pub fn is_integer(&self) -> bool {
        matches!(self, Number::Int(_) | Number::Uint(_))
    }

    /// Returns true if this is a float type.
    pub fn is_float(&self) -> bool {
        matches!(self, Number::Float(_))
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Number::Int(n)
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Self {
        Number::Uint(n)
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Number::Float(n)
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number::Int(n as i64)
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Self {
        Number::Uint(n as u64)
    }
}

impl From<f32> for Number {
    fn from(n: f32) -> Self {
        Number::Float(n as f64)
    }
}
