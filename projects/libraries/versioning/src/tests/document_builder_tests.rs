use crate::document_builder::DocumentBuilder;
use crate::output_format::OutputFormat;
use crate::release_id::ReleaseId;
use crate::tests::test_helpers::*;

#[test]
fn can_create_markdown_builder() {
    let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
    let log = basic_revision_log();
    let output = builder.generate_document(&log);

    assert!(output.contains(&format!("# Revision History: {}", TEST_PROJECT_NAME)));
}

#[test]
fn markdown_includes_release_info() {
    let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
    let mut log = basic_revision_log();

    let entry = revision_entry_with_mods(
        release_id_1_2_3(),
        vec![new_feature_mod("Test modification")],
    );
    log.append_entry(entry);

    let output = builder.generate_document(&log);
    assert!(output.contains("## Release 1.2.3"));
    assert!(output.contains("Test modification"));
}

#[test]
fn markdown_groups_by_category() {
    let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
    let mut log = basic_revision_log();

    let entry = revision_entry_with_mods(
        release_id_1_0_0(),
        vec![
            new_feature_mod("New feature added"),
            bug_fix_mod("Bug fixed"),
        ],
    );
    log.append_entry(entry);

    let output = builder.generate_document(&log);
    assert!(output.contains("### New Feature"));
    assert!(output.contains("### Fix"));
    assert!(output.contains("- New feature added"));
    assert!(output.contains("- Bug fixed"));
}

#[test]
fn markdown_includes_contributors() {
    let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
    let mut log = basic_revision_log();

    let entry = revision_entry_with_contributors(
        release_id_2_0_0(),
        vec![CONTRIBUTOR_ALICE.to_string(), CONTRIBUTOR_BOB.to_string()],
    );
    log.append_entry(entry);

    let output = builder.generate_document(&log);
    let expected_line = format!("**Contributors**: {}, {}", CONTRIBUTOR_ALICE, CONTRIBUTOR_BOB);
    assert!(output.lines().any(|line| line == expected_line));
}

#[test]
fn can_create_plaintext_builder() {
    let builder = DocumentBuilder::with_format(OutputFormat::PlainText);
    let log = basic_revision_log();
    let output = builder.generate_document(&log);

    assert!(output.contains(&format!(
        "REVISION HISTORY: {}",
        TEST_PROJECT_NAME.to_uppercase()
    )));
}

#[test]
fn plaintext_includes_release_info() {
    let builder = DocumentBuilder::with_format(OutputFormat::PlainText);
    let mut log = basic_revision_log();

    let entry = revision_entry_with_mods(
        ReleaseId::build(3, 4, 5),
        vec![enhancement_mod("Sample change")],
    );
    log.append_entry(entry);

    let output = builder.generate_document(&log);
    assert!(output.contains("Release 3.4.5"));
    assert!(output.contains("[Improvement] Sample change"));
}

#[test]
fn plaintext_shows_category_labels() {
    let builder = DocumentBuilder::with_format(OutputFormat::PlainText);
    let mut log = basic_revision_log();

    let entry =
        revision_entry_with_mods(release_id_1_0_0(), vec![security_mod(MOD_SECURITY_PATCH)]);
    log.append_entry(entry);

    let output = builder.generate_document(&log);
    assert!(output.contains(&format!("[Security] {}", MOD_SECURITY_PATCH)));
}

#[test]
fn empty_log_generates_header_only() {
    let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
    let log = revision_log_with_name("EmptyProject");
    let output = builder.generate_document(&log);

    assert!(output.contains("# Revision History: EmptyProject"));
    assert!(!output.contains("## Release"));
}

#[test]
fn multiple_releases_in_order() {
    let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
    let log = revision_log_with_entries(vec![
        basic_revision_entry(release_id_1_0_0()),
        basic_revision_entry(release_id_2_0_0()),
        basic_revision_entry(ReleaseId::build(1, 5, 0)),
    ]);

    let output = builder.generate_document(&log);
    let pos_2_0_0 = output
        .find("## Release 2.0.0")
        .expect("should find release 2.0.0");
    let pos_1_5_0 = output
        .find("## Release 1.5.0")
        .expect("should find release 1.5.0");
    let pos_1_0_0 = output
        .find("## Release 1.0.0")
        .expect("should find release 1.0.0");

    assert!(pos_2_0_0 < pos_1_5_0);
    assert!(pos_1_5_0 < pos_1_0_0);
}
