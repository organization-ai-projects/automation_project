//! tools/versioning_automation/src/pr/group_by_category.rs
pub(crate) const CATEGORIES: [&str; 9] = [
    "Security",
    "Features",
    "Bug Fixes",
    "Refactoring",
    "Automation",
    "Testing",
    "Docs",
    "Mixed",
    "Unknown",
];

pub(crate) struct GroupByCategory(
    pub(crate) u32,
    pub(crate) String,
    pub(crate) Vec<String>,
    pub(crate) usize,
);

impl GroupByCategory {
    pub(crate) fn parse_records(text: &str) -> Vec<Self> {
        let mut out = Vec::new();
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let parts = trimmed
                .split('|')
                .map(|part| part.to_string())
                .collect::<Vec<String>>();
            if parts.len() < 2 {
                continue;
            }
            let issue_number = parts[0].parse::<u32>().unwrap_or(u32::MAX);
            let category = parts[1].clone();
            let fields = parts.into_iter().skip(2).collect::<Vec<String>>();
            out.push(Self(issue_number, category, fields, index));
        }
        out
    }

    pub(crate) fn render_grouped_output(records: &[Self], mode: &str) -> String {
        let mut out = String::new();
        for category in CATEGORIES {
            let matching = records
                .iter()
                .filter(|record| record.1 == category)
                .collect::<Vec<&GroupByCategory>>();
            if matching.is_empty() {
                continue;
            }

            out.push_str("#### ");
            out.push_str(category);
            out.push('\n');

            for record in matching {
                out.push_str(&Self::render_line(record, mode));
                out.push('\n');
            }
            out.push('\n');
        }
        out
    }

    pub(crate) fn render_line(record: &Self, mode: &str) -> String {
        let action = record.2.first().cloned().unwrap_or_default();
        let issue_key = record.2.get(1).cloned().unwrap_or_default();

        if mode == "resolved" {
            return format!("- {action} {issue_key}");
        }
        if mode == "reopen" {
            return format!("- Reopen {action}");
        }
        if mode == "conflict" {
            return format!("- {action} - {issue_key}");
        }
        format!("- {action}")
    }

    pub(crate) fn render_issue_outcome_groups(records: &[Self]) -> String {
        Self::render_issue_outcome_groups_with_mode(records, "resolved")
    }

    pub(crate) fn render_issue_outcome_groups_with_mode(records: &[Self], mode: &str) -> String {
        let mut out = String::new();
        for category in CATEGORIES {
            let matching = records
                .iter()
                .filter(|record| record.1 == category)
                .collect::<Vec<_>>();
            if matching.is_empty() {
                continue;
            }

            out.push_str("#### ");
            out.push_str(category);
            out.push('\n');

            for record in matching {
                let action = record.2.first().cloned().unwrap_or_default();
                let issue_key = record.2.get(1).cloned().unwrap_or_default();
                let line = if mode == "conflict" {
                    format!("- {action} - {issue_key}")
                } else if mode == "directive" {
                    format!("- {action}")
                } else {
                    format!("- {action} {issue_key}")
                };
                out.push_str(&line);
                out.push('\n');
            }
            out.push('\n');
        }
        out
    }
}
