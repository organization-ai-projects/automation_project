//! projects/products/stable/platform_ide/backend/src/auth/tests/session.rs
use crate::auth::Session;

#[test]
fn session_redacts_token_in_debug() {
    let session = Session::new("secret-token", "user-1");
    let dbg = format!("{session:?}");

    assert!(dbg.contains("user-1"));
    assert!(dbg.contains("[REDACTED]"));
    assert!(!dbg.contains("secret-token"));
}
