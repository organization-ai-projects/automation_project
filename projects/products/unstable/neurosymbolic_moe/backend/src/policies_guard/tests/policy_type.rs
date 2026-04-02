//! projects/products/unstable/neurosymbolic_moe/backend/src/policy_guard/tests/policy_type.rs
use crate::policies_guard::PolicyType;

#[test]
fn policy_type_variants_are_constructible() {
    let a = PolicyType::ContentFilter;
    let b = PolicyType::SafetyCheck;
    let c = PolicyType::FormatValidation;
    let d = PolicyType::LengthLimit(42);
    let e = PolicyType::Custom("custom".to_string());
    assert!(matches!(a, PolicyType::ContentFilter));
    assert!(matches!(b, PolicyType::SafetyCheck));
    assert!(matches!(c, PolicyType::FormatValidation));
    assert!(matches!(d, PolicyType::LengthLimit(42)));
    assert!(matches!(e, PolicyType::Custom(_)));
}
