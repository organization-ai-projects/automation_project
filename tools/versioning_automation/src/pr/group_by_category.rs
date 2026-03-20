use crate::pr::commands::pr_group_by_category_options::PrGroupByCategoryOptions;

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

pub(crate) fn run_group_by_category(opts: PrGroupByCategoryOptions) -> i32 {
    let mode = opts.mode.as_str();
    if !matches!(mode, "resolved" | "reopen" | "conflict" | "directive") {
        eprintln!("--mode must be one of: resolved, reopen, conflict, directive");
        return 2;
    }

    let mut records = parse_records(&opts.text);
    records.sort_by_key(|record| (record.0, record.3));

    let output = render_grouped_output(&records, mode);
    print!("{output}");
    0
}

pub(crate) type GroupByCategory = (u32, String, Vec<String>, usize);

pub(crate) fn parse_records(text: &str) -> Vec<GroupByCategory> {
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
        out.push((issue_number, category, fields, index));
    }
    out
}

pub(crate) fn render_grouped_output(records: &[GroupByCategory], mode: &str) -> String {
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
            out.push_str(&render_line(record, mode));
            out.push('\n');
        }
        out.push('\n');
    }
    out
}

fn render_line(record: &GroupByCategory, mode: &str) -> String {
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
