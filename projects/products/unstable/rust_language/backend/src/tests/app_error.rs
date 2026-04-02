//! projects/products/unstable/rust_language/backend/src/tests/app_errors.rs
use std::io;

use crate::app_error::AppError;

#[test]
fn test_app_error_display() {
    let io_error = io::Error::new(io::ErrorKind::Other, "test");
    let app_error = AppError::from(io_error);
    assert_eq!(format!("{}", app_error), "I/O error: test");
}
