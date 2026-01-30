use crate::{hash_password, verify_password};

#[test]
fn test_password_hashing() {
    let password = "my_secure_password";
    let hash = hash_password(password).expect("hash password for test");

    assert!(verify_password(password, &hash).expect("verify password for test"));
    assert!(!verify_password("wrong_password", &hash).expect("verify wrong password for test"));
}
