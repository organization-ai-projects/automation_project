use crate::release_id::ReleaseId;

#[test]
fn can_create_release_id() {
    let release = ReleaseId::build(2, 5, 8);
    assert_eq!(release.first_tier(), 2);
    assert_eq!(release.second_tier(), 5);
    assert_eq!(release.third_tier(), 8);
}

#[test]
fn initial_release_is_correct() {
    let release = ReleaseId::initial();
    assert_eq!(release.first_tier(), 1);
    assert_eq!(release.second_tier(), 0);
    assert_eq!(release.third_tier(), 0);
}

#[test]
fn can_format_as_string() {
    let release = ReleaseId::build(3, 7, 12);
    assert_eq!(release.to_string(), "3.7.12");
}

#[test]
fn can_parse_from_string() {
    let parsed = ReleaseId::parse_str("4.2.1").unwrap();
    assert_eq!(parsed.first_tier(), 4);
    assert_eq!(parsed.second_tier(), 2);
    assert_eq!(parsed.third_tier(), 1);
}

#[test]
fn roundtrip_conversion() {
    let original = ReleaseId::build(6, 11, 3);
    let as_string = original.to_string();
    let parsed: ReleaseId = as_string.parse().unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn major_advancement_resets_others() {
    let release = ReleaseId::build(2, 5, 8);
    let next = release.advance_major();
    assert_eq!(next.first_tier(), 3);
    assert_eq!(next.second_tier(), 0);
    assert_eq!(next.third_tier(), 0);
}

#[test]
fn feature_advancement_resets_corrections() {
    let release = ReleaseId::build(2, 5, 8);
    let next = release.advance_feature();
    assert_eq!(next.first_tier(), 2);
    assert_eq!(next.second_tier(), 6);
    assert_eq!(next.third_tier(), 0);
}

#[test]
fn correction_advancement_preserves_others() {
    let release = ReleaseId::build(2, 5, 8);
    let next = release.advance_correction();
    assert_eq!(next.first_tier(), 2);
    assert_eq!(next.second_tier(), 5);
    assert_eq!(next.third_tier(), 9);
}

#[test]
fn detects_breaking_compatibility() {
    let v1 = ReleaseId::build(2, 5, 8);
    let v2 = ReleaseId::build(3, 0, 0);
    assert!(v2.breaks_compatibility_with(&v1));
}

#[test]
fn compatible_versions_detected() {
    let v1 = ReleaseId::build(2, 5, 8);
    let v2 = ReleaseId::build(2, 7, 3);
    assert!(!v2.breaks_compatibility_with(&v1));
}

#[test]
fn ordering_works_correctly() {
    let v1 = ReleaseId::build(1, 5, 3);
    let v2 = ReleaseId::build(2, 0, 0);
    let v3 = ReleaseId::build(2, 1, 0);

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);
}

#[test]
fn invalid_format_rejected() {
    assert!(ReleaseId::parse_str("1.2").is_err());
    assert!(ReleaseId::parse_str("1.2.3.4").is_err());
    assert!(ReleaseId::parse_str("abc.def.ghi").is_err());
}
