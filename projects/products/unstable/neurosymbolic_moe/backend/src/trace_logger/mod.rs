//! projects/products/unstable/neurosymbolic_moe/backend/src/trace_logger/mod.rs
#[path = "trace_logger.rs"]
mod trace_logger_core;

pub use trace_logger_core::TraceLogger;

#[cfg(test)]
mod tests;
