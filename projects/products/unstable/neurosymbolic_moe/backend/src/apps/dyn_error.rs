//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/dyn_error.rs
use std::error;

pub(crate) type DynError = Box<dyn error::Error>;
