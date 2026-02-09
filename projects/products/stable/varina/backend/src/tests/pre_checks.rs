// projects/products/stable/varina/backend/src/tests/pre_checks.rs
use crate::pre_checks::PreChecks;

#[test]
fn test_pre_checks_usage() {
    let check = PreChecks::None;
    assert_eq!(check, PreChecks::None);

    let check = PreChecks::FmtCheck;
    assert_eq!(check, PreChecks::FmtCheck);

    let check = PreChecks::FmtCheckAndTests;
    assert_eq!(check, PreChecks::FmtCheckAndTests);
}
