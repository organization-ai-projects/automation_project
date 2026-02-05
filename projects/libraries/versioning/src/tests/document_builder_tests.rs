#[cfg(test)]
mod tests {
    use crate::document_builder::{DocumentBuilder, OutputFormat};
    use crate::release_id::ReleaseId;
    use crate::revision_log::{
        ModificationCategory, ModificationEntry, RevisionEntry, RevisionLog,
    };
    use chrono::Utc;

    #[test]
    fn can_create_markdown_builder() {
        let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
        let log = RevisionLog::initialize("TestProject".to_string());
        let output = builder.generate_document(&log);

        assert!(output.contains("# Revision History: TestProject"));
    }

    #[test]
    fn markdown_includes_release_info() {
        let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
        let mut log = RevisionLog::initialize("TestProject".to_string());

        let mut entry = RevisionEntry::create(ReleaseId::build(1, 2, 3), Utc::now());
        entry.append_modification(ModificationEntry::create(
            "Test modification".to_string(),
            ModificationCategory::NewCapability,
        ));
        log.append_entry(entry);

        let output = builder.generate_document(&log);
        assert!(output.contains("## Release 1.2.3"));
        assert!(output.contains("Test modification"));
    }

    #[test]
    fn markdown_groups_by_category() {
        let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
        let mut log = RevisionLog::initialize("TestProject".to_string());

        let mut entry = RevisionEntry::create(ReleaseId::build(1, 0, 0), Utc::now());
        entry.append_modification(ModificationEntry::create(
            "New feature added".to_string(),
            ModificationCategory::NewCapability,
        ));
        entry.append_modification(ModificationEntry::create(
            "Bug fixed".to_string(),
            ModificationCategory::CorrectionApplied,
        ));
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
        let mut log = RevisionLog::initialize("TestProject".to_string());

        let mut entry = RevisionEntry::create(ReleaseId::build(2, 0, 0), Utc::now());
        entry.append_contributor("Alice".to_string());
        entry.append_contributor("Bob".to_string());
        log.append_entry(entry);

        let output = builder.generate_document(&log);
        assert!(output.contains("**Contributors**: Alice, Bob"));
    }

    #[test]
    fn can_create_plaintext_builder() {
        let builder = DocumentBuilder::with_format(OutputFormat::PlainText);
        let log = RevisionLog::initialize("TestProject".to_string());
        let output = builder.generate_document(&log);

        assert!(output.contains("REVISION HISTORY: TESTPROJECT"));
    }

    #[test]
    fn plaintext_includes_release_info() {
        let builder = DocumentBuilder::with_format(OutputFormat::PlainText);
        let mut log = RevisionLog::initialize("TestProject".to_string());

        let mut entry = RevisionEntry::create(ReleaseId::build(3, 4, 5), Utc::now());
        entry.append_modification(ModificationEntry::create(
            "Sample change".to_string(),
            ModificationCategory::Enhancement,
        ));
        log.append_entry(entry);

        let output = builder.generate_document(&log);
        assert!(output.contains("Release 3.4.5"));
        assert!(output.contains("[Improvement] Sample change"));
    }

    #[test]
    fn plaintext_shows_category_labels() {
        let builder = DocumentBuilder::with_format(OutputFormat::PlainText);
        let mut log = RevisionLog::initialize("TestProject".to_string());

        let mut entry = RevisionEntry::create(ReleaseId::build(1, 0, 0), Utc::now());
        entry.append_modification(ModificationEntry::create(
            "Security patch".to_string(),
            ModificationCategory::SecurityUpdate,
        ));
        log.append_entry(entry);

        let output = builder.generate_document(&log);
        assert!(output.contains("[Security] Security patch"));
    }

    #[test]
    fn empty_log_generates_header_only() {
        let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
        let log = RevisionLog::initialize("EmptyProject".to_string());
        let output = builder.generate_document(&log);

        assert!(output.contains("# Revision History: EmptyProject"));
        assert!(!output.contains("## Release"));
    }

    #[test]
    fn multiple_releases_in_order() {
        let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
        let mut log = RevisionLog::initialize("TestProject".to_string());

        log.append_entry(RevisionEntry::create(ReleaseId::build(1, 0, 0), Utc::now()));
        log.append_entry(RevisionEntry::create(ReleaseId::build(2, 0, 0), Utc::now()));
        log.append_entry(RevisionEntry::create(ReleaseId::build(1, 5, 0), Utc::now()));

        let output = builder.generate_document(&log);
        let pos_2_0_0 = output.find("## Release 2.0.0").unwrap();
        let pos_1_5_0 = output.find("## Release 1.5.0").unwrap();
        let pos_1_0_0 = output.find("## Release 1.0.0").unwrap();

        assert!(pos_2_0_0 < pos_1_5_0);
        assert!(pos_1_5_0 < pos_1_0_0);
    }
}
