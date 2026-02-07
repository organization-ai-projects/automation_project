use crate::modification_category::ModificationCategory;
use crate::modification_entry::ModificationEntry;
use crate::release_id::ReleaseId;
use crate::revision_entry::RevisionEntry;
use crate::tests::test_helpers::*;
use chrono::Utc;

#[test]
fn can_create_modification_entry() {
    let entry = ModificationEntry::create(
        "Add new authentication system".to_string(),
        ModificationCategory::NewCapability,
    );
    assert_eq!(entry.get_description(), "Add new authentication system");
    assert_eq!(entry.get_category(), &ModificationCategory::NewCapability);
}

#[test]
fn category_labels_are_correct() {
    assert_eq!(
        ModificationCategory::BreakingModification.label(),
        "Breaking Change"
    );
    assert_eq!(ModificationCategory::NewCapability.label(), "New Feature");
    assert_eq!(ModificationCategory::Enhancement.label(), "Improvement");
    assert_eq!(ModificationCategory::CorrectionApplied.label(), "Fix");
    assert_eq!(ModificationCategory::SecurityUpdate.label(), "Security");
    assert_eq!(
        ModificationCategory::DeprecationNotice.label(),
        "Deprecated"
    );
}

#[test]
fn can_create_revision_entry() {
    let release = release_id_1_2_3();
    let timestamp = Utc::now();
    let entry = RevisionEntry::create(release, timestamp);

    assert_eq!(entry.get_release(), &release);
    assert_eq!(entry.get_modifications().len(), 0);
    assert_eq!(entry.get_contributors().len(), 0);
}

#[test]
fn can_append_modifications() {
    let release = release_id_1_0_0();
    let mut entry = RevisionEntry::create(release, Utc::now());

    entry.append_modification(new_feature_mod("First change"));
    entry.append_modification(bug_fix_mod("Second change"));

    assert_eq!(entry.get_modifications().len(), 2);
}

#[test]
fn can_append_contributors() {
    let release = release_id_1_0_0();
    let mut entry = RevisionEntry::create(release, Utc::now());

    entry.append_contributor(CONTRIBUTOR_ALICE.to_string());
    entry.append_contributor(CONTRIBUTOR_BOB.to_string());

    assert_eq!(entry.get_contributors().len(), 2);
    assert!(entry
        .get_contributors()
        .iter()
        .any(|c| c == CONTRIBUTOR_ALICE));
}

#[test]
fn duplicate_contributors_not_added() {
    let release = release_id_1_0_0();
    let mut entry = RevisionEntry::create(release, Utc::now());

    entry.append_contributor(CONTRIBUTOR_ALICE.to_string());
    entry.append_contributor(CONTRIBUTOR_ALICE.to_string());

    assert_eq!(entry.get_contributors().len(), 1);
}

#[test]
fn can_initialize_revision_log() {
    let log = basic_revision_log();
    assert_eq!(log.get_project_title(), TEST_PROJECT_NAME);
    assert_eq!(log.get_entries().len(), 0);
}

#[test]
fn can_append_entries_to_log() {
    let mut log = basic_revision_log();

    let entry1 = basic_revision_entry(release_id_1_0_0());
    let entry2 = basic_revision_entry(ReleaseId::build(1, 1, 0));

    log.append_entry(entry1);
    log.append_entry(entry2);

    assert_eq!(log.get_entries().len(), 2);
}

#[test]
fn entries_are_sorted_descending() {
    let mut log = basic_revision_log();

    log.append_entry(basic_revision_entry(release_id_1_0_0()));
    log.append_entry(basic_revision_entry(release_id_2_0_0()));
    log.append_entry(basic_revision_entry(ReleaseId::build(1, 5, 0)));

    let entries = log.get_entries();
    assert_eq!(entries[0].get_release().first_tier(), 2);
    assert_eq!(entries[1].get_release().second_tier(), 5);
    assert_eq!(entries[2].get_release().second_tier(), 0);
}

#[test]
fn can_find_specific_entry() {
    let mut log = basic_revision_log();
    let target_release = ReleaseId::build(1, 5, 3);

    log.append_entry(basic_revision_entry(release_id_1_0_0()));
    log.append_entry(basic_revision_entry(target_release));
    log.append_entry(basic_revision_entry(release_id_2_0_0()));

    let found = log.find_entry(&target_release);
    assert!(found.is_some());
    assert_eq!(found.expect("entry should exist").get_release(), &target_release);
}

#[test]
fn most_recent_returns_highest_version() {
    let mut log = basic_revision_log();

    log.append_entry(basic_revision_entry(release_id_1_0_0()));
    log.append_entry(basic_revision_entry(ReleaseId::build(2, 5, 0)));
    log.append_entry(basic_revision_entry(release_id_2_0_0()));

    let recent = log.most_recent().expect("log should have entries");
    assert_eq!(recent.get_release().first_tier(), 2);
    assert_eq!(recent.get_release().second_tier(), 5);
}
