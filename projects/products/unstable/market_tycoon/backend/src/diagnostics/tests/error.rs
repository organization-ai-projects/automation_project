use crate::diagnostics::error::Error;

#[test]
fn display_invalid_cli() {
    let e = Error::InvalidCli("bad flag".into());
    assert_eq!(e.to_string(), "invalid CLI: bad flag");
}

#[test]
fn display_replay_mismatch() {
    let e = Error::ReplayMismatch("hash differs".into());
    assert_eq!(e.to_string(), "replay mismatch: hash differs");
}
