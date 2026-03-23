#[test]
fn module_compiles() {
    let value = crate::nonce::next_nonce();
    assert!(value > 0);
}
