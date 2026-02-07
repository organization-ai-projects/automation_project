use crate::release_id::ReleaseId;
use crate::release_tracker::ReleaseTracker;
use crate::tests::test_helpers::*;
use tempfile::NamedTempFile;

#[test]
fn can_initialize_tracker() {
    let tracker = ReleaseTracker::initialize(ALT_PROJECT_NAME.to_string());
    assert_eq!(tracker.active_release(), &ReleaseId::initial());
    assert_eq!(tracker.log().get_project_title(), ALT_PROJECT_NAME);
}

#[test]
fn initial_tracker_has_first_entry() {
    let tracker = ReleaseTracker::initialize(ALT_PROJECT_NAME.to_string());
    let entries = tracker.log().get_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].get_release(), &ReleaseId::initial());
}

#[test]
fn can_register_major_release() {
    let mut tracker = ReleaseTracker::initialize(ALT_PROJECT_NAME.to_string());

    let mods = vec![breaking_mod("Complete API redesign")];

    tracker.register_major_release(mods, vec![CONTRIBUTOR_DEV_A.to_string()]);

    assert_eq!(tracker.active_release().first_tier(), 2);
    assert_eq!(tracker.active_release().second_tier(), 0);
    assert_eq!(tracker.active_release().third_tier(), 0);
}

#[test]
fn can_register_feature_release() {
    let mut tracker = ReleaseTracker::initialize(ALT_PROJECT_NAME.to_string());

    let mods = vec![new_feature_mod("Add user dashboard")];

    tracker.register_feature_release(mods, vec![CONTRIBUTOR_DEV_B.to_string()]);

    assert_eq!(tracker.active_release().first_tier(), 1);
    assert_eq!(tracker.active_release().second_tier(), 1);
    assert_eq!(tracker.active_release().third_tier(), 0);
}

#[test]
fn can_register_correction_release() {
    let mut tracker = ReleaseTracker::initialize(ALT_PROJECT_NAME.to_string());

    let mods = vec![bug_fix_mod("Fix login issue")];

    tracker.register_correction_release(mods, vec![CONTRIBUTOR_DEV_C.to_string()]);

    assert_eq!(tracker.active_release().first_tier(), 1);
    assert_eq!(tracker.active_release().second_tier(), 0);
    assert_eq!(tracker.active_release().third_tier(), 1);
}

#[test]
fn multiple_releases_tracked() {
    let mut tracker = ReleaseTracker::initialize(ALT_PROJECT_NAME.to_string());

    tracker.register_feature_release(vec![new_feature_mod("Feature 1")], vec![]);
    tracker.register_feature_release(vec![new_feature_mod("Feature 2")], vec![]);
    tracker.register_correction_release(vec![bug_fix_mod(MOD_BUG_FIX)], vec![]);

    assert_eq!(tracker.active_release().to_string(), "1.2.1");
    assert_eq!(tracker.log().get_entries().len(), 4); // Initial + 3 new releases
}

#[test]
fn can_persist_and_load_tracker() {
    let temp_file = NamedTempFile::new().expect("failed to create temp file");
    let temp_path = temp_file.path();

    let mut tracker = ReleaseTracker::initialize(TEST_PROJECT_NAME.to_string());
    tracker.register_feature_release(
        vec![new_feature_mod(MOD_NEW_FEATURE)],
        vec!["Dev1".to_string()],
    );

    tracker
        .persist_to_file(temp_path)
        .expect("failed to persist tracker");

    let loaded = ReleaseTracker::load_from_file(temp_path).expect("failed to load tracker");
    assert_eq!(loaded.active_release(), tracker.active_release());
    assert_eq!(
        loaded.log().get_entries().len(),
        tracker.log().get_entries().len()
    );
}

#[test]
fn loaded_tracker_preserves_history() {
    let temp_file = NamedTempFile::new().expect("failed to create temp file");
    let temp_path = temp_file.path();

    let mut original = ReleaseTracker::initialize(TEST_PROJECT_NAME.to_string());
    original.register_major_release(
        vec![breaking_mod(MOD_BREAKING_CHANGE)],
        vec![CONTRIBUTOR_ALICE.to_string()],
    );
    original.register_feature_release(
        vec![new_feature_mod(MOD_NEW_FEATURE)],
        vec![CONTRIBUTOR_BOB.to_string()],
    );

    original
        .persist_to_file(temp_path)
        .expect("failed to persist tracker");
    let loaded = ReleaseTracker::load_from_file(temp_path).expect("failed to load tracker");

    assert_eq!(loaded.log().get_entries().len(), 3); // Initial + 2 new
    assert_eq!(loaded.active_release().to_string(), "2.1.0");
}
