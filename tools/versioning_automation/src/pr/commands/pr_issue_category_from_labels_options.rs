//! tools/versioning_automation/src/pr/commands/pr_issue_category_from_labels_options.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrIssueCategoryFromLabelsOptions {
    pub(crate) labels_raw: String,
}
