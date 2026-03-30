//! tools/versioning_automation/src/parent_field.rs
use crate::lazy_regex::PARENT_FIELD_REGEX;

pub(crate) fn extract_parent_field(body: &str) -> Option<String> {
    let re = PARENT_FIELD_REGEX.as_ref().ok()?;
    let mut parent_value: Option<String> = None;

    for line in body.lines() {
        if let Some(captures) = re.captures(line) {
            parent_value = captures.get(1).map(|m| m.as_str().trim().to_lowercase());
        }
    }

    parent_value.map(|raw| {
        raw.trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string()
    })
}
