use regex::Regex;

use crate::pr::commands::pr_effective_category_options::PrEffectiveCategoryOptions;
use crate::pr::commands::pr_issue_category_from_labels_options::PrIssueCategoryFromLabelsOptions;
use crate::pr::commands::pr_issue_category_from_title_options::PrIssueCategoryFromTitleOptions;
use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;

pub(crate) fn run_resolve_category(opts: PrResolveCategoryOptions) -> i32 {
    let effective = resolve_effective_category(
        &opts.label_category,
        &opts.title_category,
        &opts.default_category,
    );

    println!("{effective}");
    0
}

pub(crate) fn run_effective_category(opts: PrEffectiveCategoryOptions) -> i32 {
    let label_category = issue_category_from_labels(&opts.labels_raw);
    let title_category = if let Some(title) = &opts.title {
        issue_category_from_title(title)
    } else if let Some(title_category) = &opts.title_category {
        title_category.as_str()
    } else {
        "Unknown"
    };
    let effective =
        resolve_effective_category(label_category, title_category, &opts.default_category);
    println!("{effective}");
    0
}

pub(crate) fn run_issue_category_from_labels(opts: PrIssueCategoryFromLabelsOptions) -> i32 {
    println!("{}", issue_category_from_labels(&opts.labels_raw));
    0
}

pub(crate) fn run_issue_category_from_title(opts: PrIssueCategoryFromTitleOptions) -> i32 {
    println!("{}", issue_category_from_title(&opts.title));
    0
}

fn resolve_effective_category(
    label_category: &str,
    title_category: &str,
    default_category: &str,
) -> String {
    let mut effective = label_category.to_string();
    if (effective == "Unknown" || effective == "Mixed")
        && title_category != "Unknown"
        && title_category != "Mixed"
    {
        effective = title_category.to_string();
    }
    if effective == "Unknown" && default_category != "Unknown" {
        effective = default_category.to_string();
    }
    effective
}

fn issue_shared_priority_category(
    has_security: bool,
    has_automation: bool,
    has_testing: bool,
    has_docs: bool,
) -> Option<&'static str> {
    if has_security {
        return Some("Security");
    }
    if has_automation {
        return Some("Automation");
    }
    if has_testing {
        return Some("Testing");
    }
    if has_docs {
        return Some("Docs");
    }
    None
}

fn issue_category_from_labels(labels_raw: &str) -> &'static str {
    let mut has_security = false;
    let mut has_bug = false;
    let mut has_refactor = false;
    let mut has_feature = false;
    let mut has_testing = false;
    let mut has_automation = false;
    let mut has_docs = false;

    for label in labels_raw.to_lowercase().split("||") {
        if label.is_empty() {
            continue;
        }
        match label {
            "security" | "sec" | "codeql" | "cve" | "vuln" | "vulnerability" | "sast" => {
                has_security = true;
            }
            "bug" | "defect" | "regression" | "incident" => has_bug = true,
            "refactor" | "cleanup" | "chore" | "maintainability" | "maintenance" | "tech-debt"
            | "tech_debt" | "technical-debt" | "technical_debt" => has_refactor = true,
            "feature" | "enhancement" | "feat" => has_feature = true,
            "testing" | "tests" | "test" => has_testing = true,
            "automation" | "automation-failed" | "sync_branch" | "scripts" | "linting"
            | "workflow" | "ci" => has_automation = true,
            "documentation" | "docs" | "readme" | "translation" => has_docs = true,
            _ => {}
        }
    }

    if let Some(priority) =
        issue_shared_priority_category(has_security, has_automation, has_testing, has_docs)
    {
        return priority;
    }

    let count = (has_bug as u8) + (has_refactor as u8) + (has_feature as u8);
    if count >= 2 {
        return "Mixed";
    }
    if has_bug {
        return "Bug Fixes";
    }
    if has_refactor {
        return "Refactoring";
    }
    if has_feature {
        return "Features";
    }
    "Unknown"
}

fn issue_category_from_title(title: &str) -> &'static str {
    let title_lc = title.to_lowercase();
    let security_re =
        Regex::new(r"(^|[^a-z])(security|vuln|vulnerability|cve|sast|codeql)([^a-z]|$)")
            .expect("valid regex");
    let automation_re = Regex::new(r"(^|[^a-z])(automation|workflow|ci|script|lint)([^a-z]|$)")
        .expect("valid regex");
    let testing_re = Regex::new(r"(^|[^a-z])(test|tests|testing)([^a-z]|$)").expect("valid regex");
    let docs_re =
        Regex::new(r"(^|[^a-z])(docs|documentation|readme)([^a-z]|$)").expect("valid regex");

    let has_security = security_re.is_match(&title_lc);
    let has_automation = automation_re.is_match(&title_lc);
    let has_testing = testing_re.is_match(&title_lc);
    let has_docs = docs_re.is_match(&title_lc);

    if let Some(priority) =
        issue_shared_priority_category(has_security, has_automation, has_testing, has_docs)
    {
        return priority;
    }

    let fix_re = Regex::new(r"^fix(\(|:|!|[[:space:]])").expect("valid regex");
    let bug_re = Regex::new(r"(^|[^a-z])(bug|hotfix|regression|error|failure)([^a-z]|$)")
        .expect("valid regex");
    if fix_re.is_match(&title_lc) || bug_re.is_match(&title_lc) {
        return "Bug Fixes";
    }

    let refactor_re = Regex::new(r"^(chore|refactor)(\(|:|!|[[:space:]])").expect("valid regex");
    let maintain_re =
        Regex::new(r"(^|[^a-z])(cleanup|maintainability|tech[[:space:]_-]?debt)([^a-z]|$)")
            .expect("valid regex");
    if refactor_re.is_match(&title_lc) || maintain_re.is_match(&title_lc) {
        return "Refactoring";
    }

    let feat_re = Regex::new(r"^feat(\(|:|!|[[:space:]])").expect("valid regex");
    let enhancement_re =
        Regex::new(r"(^|[^a-z])(feature|enhancement)([^a-z]|$)").expect("valid regex");
    if feat_re.is_match(&title_lc) || enhancement_re.is_match(&title_lc) {
        return "Features";
    }

    "Unknown"
}
