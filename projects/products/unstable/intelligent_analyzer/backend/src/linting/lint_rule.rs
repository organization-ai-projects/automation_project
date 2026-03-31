use super::lint_rule_id::LintRuleId;
use crate::analysis::Finding;
use crate::analysis::FindingKind;
use crate::analysis::Severity;

/// A single lint rule that can check source code.
pub struct LintRule {
    pub id: LintRuleId,
}

impl LintRule {
    pub fn check(&self, source: &str) -> Vec<Finding> {
        match &self.id {
            LintRuleId::TrailingWhitespace => check_trailing_whitespace(source),
            LintRuleId::LineTooLong => check_line_too_long(source),
            LintRuleId::TodoComment => check_todo_comments(source),
            LintRuleId::UnusedImport => check_unused_imports(source),
            LintRuleId::MissingDocComment => check_missing_doc_comments(source),
            LintRuleId::Custom(_) => Vec::new(),
        }
    }
}

fn check_trailing_whitespace(source: &str) -> Vec<Finding> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| line != &line.trim_end())
        .map(|(idx, _)| {
            Finding::new(
                FindingKind::Custom("trailing_whitespace".to_string()),
                Severity::Hint,
                idx + 1,
                "trailing whitespace".to_string(),
            )
        })
        .collect()
}

fn check_line_too_long(source: &str) -> Vec<Finding> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| line.len() > 120)
        .map(|(idx, line)| {
            Finding::new(
                FindingKind::Custom("line_too_long".to_string()),
                Severity::Hint,
                idx + 1,
                format!("line is {} characters (max 120)", line.len()),
            )
        })
        .collect()
}

fn check_todo_comments(source: &str) -> Vec<Finding> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let trimmed = line.trim();
            trimmed.contains("TODO") || trimmed.contains("FIXME") || trimmed.contains("HACK")
        })
        .map(|(idx, _)| {
            Finding::new(
                FindingKind::Custom("todo_comment".to_string()),
                Severity::Hint,
                idx + 1,
                "TODO/FIXME/HACK comment detected".to_string(),
            )
        })
        .collect()
}

fn check_unused_imports(source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut imports: Vec<(String, usize)> = Vec::new();

    for (idx, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("use ") {
            if let Some(name) = rest.split("::").last() {
                let name = name.trim_end_matches(';').trim();
                if !name.is_empty() && !name.contains('{') {
                    imports.push((name.to_string(), idx + 1));
                }
            }
        }
    }

    let body: String = source
        .lines()
        .filter(|l| !l.trim().starts_with("use "))
        .collect::<Vec<_>>()
        .join("\n");

    for (name, line) in imports {
        if !body.contains(&name) {
            findings.push(Finding::new(
                FindingKind::Custom("unused_import".to_string()),
                Severity::Warning,
                line,
                format!("import `{name}` appears unused"),
            ));
        }
    }

    findings
}

fn check_missing_doc_comments(source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub struct ") {
            let has_doc = idx > 0
                && lines[..idx]
                    .iter()
                    .rev()
                    .take_while(|l| {
                        let t = l.trim();
                        t.starts_with("///") || t.starts_with("#[")
                    })
                    .any(|l| l.trim().starts_with("///"));
            if !has_doc {
                findings.push(Finding::new(
                    FindingKind::Custom("missing_doc_comment".to_string()),
                    Severity::Hint,
                    idx + 1,
                    "public item lacks a doc comment".to_string(),
                ));
            }
        }
    }

    findings
}
