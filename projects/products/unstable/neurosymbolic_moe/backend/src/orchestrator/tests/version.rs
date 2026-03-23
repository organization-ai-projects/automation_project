use crate::orchestrator::Version;

#[test]
fn test_version_increment_major() {
    let mut version = Version::new(1, 2, 3);
    version.increment_major();
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
}

#[test]
fn test_version_increment_minor() {
    let mut version = Version::new(1, 2, 3);
    version.increment_minor();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 3);
    assert_eq!(version.patch, 0);
}

#[test]
fn test_version_increment_patch() {
    let mut version = Version::new(1, 2, 3);
    version.increment_patch();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 4);
}

#[test]
fn test_version_to_string() {
    let version = Version::new(1, 2, 3);
    assert_eq!(version.to_string(), "1.2.3");
}

#[test]
fn test_version_default() {
    let version = Version::default();
    assert_eq!(version.major, 0);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 1);
}

#[test]
fn test_version_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 1, 0);
    let v3 = Version::new(2, 0, 0);

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v3 > v1);
}
