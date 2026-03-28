//! tools/versioning_automation/src/issues/required_fields/labels.rs
pub(crate) fn profile_for_labels(labels_raw: &str) -> &'static str {
    if labels_include(labels_raw, "review") {
        return "review";
    }
    "default"
}

pub(crate) fn labels_include(labels_raw: &str, expected: &str) -> bool {
    labels_raw
        .split("||")
        .any(|label| label.trim().eq_ignore_ascii_case(expected))
}
