//! tools/versioning_automation/src/issues/tasklist_refs.rs
use regex::Regex;

pub(crate) fn extract_tasklist_refs(body: &str) -> Vec<String> {
    let issue_ref_regex = match Regex::new(r"#([0-9]+)") {
        Ok(regex) => regex,
        Err(_) => return Vec::new(),
    };

    let mut refs: Vec<String> = body
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("- [ ]")
                || trimmed.starts_with("- [x]")
                || trimmed.starts_with("- [X]")
        })
        .flat_map(|line| {
            issue_ref_regex
                .captures_iter(line)
                .filter_map(|captures| captures.get(1).map(|m| format!("#{}", m.as_str())))
                .collect::<Vec<_>>()
        })
        .collect();

    refs.sort();
    refs.dedup();
    refs
}
