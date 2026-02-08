use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueCategory {
    Security,
    Features,
    BugFixes,
    Refactoring,
    Automation,
    Testing,
    Docs,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorySource {
    DeterministicLabels,
    LatentHeuristic,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDecision {
    pub category: IssueCategory,
    pub source: CategorySource,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueClassificationInput {
    pub labels: Vec<String>,
    pub title: String,
    pub body: String,
}

pub fn classify_issue(input: &IssueClassificationInput, latent_threshold: f64) -> CategoryDecision {
    if let Some(category) = classify_from_labels(&input.labels) {
        return CategoryDecision {
            category,
            source: CategorySource::DeterministicLabels,
            confidence: 1.0,
        };
    }

    let (category, confidence) = classify_latent(&input.title, &input.body);
    if confidence >= latent_threshold {
        CategoryDecision {
            category,
            source: CategorySource::LatentHeuristic,
            confidence,
        }
    } else {
        CategoryDecision {
            category: IssueCategory::Unknown,
            source: CategorySource::Unknown,
            confidence,
        }
    }
}

fn classify_from_labels(labels: &[String]) -> Option<IssueCategory> {
    let joined = labels.join("||").to_lowercase();
    let has_security = contains_any(
        &joined,
        &["security", "sec", "codeql", "cve", "vuln", "sast"],
    );
    let has_bug = contains_any(&joined, &["bug", "defect", "regression", "incident"]);
    let has_refactor = contains_any(
        &joined,
        &["refactor", "cleanup", "chore", "maintenance", "tech debt"],
    );
    let has_feature = contains_any(&joined, &["feature", "enhancement", "feat"]);
    let has_testing = contains_any(&joined, &["testing", "tests", "test"]);
    let has_automation = contains_any(
        &joined,
        &[
            "automation",
            "automation-failed",
            "sync_branch",
            "scripts",
            "linting",
            "workflow",
            "ci",
        ],
    );
    let has_docs = contains_any(&joined, &["documentation", "docs", "readme", "translation"]);

    if has_security {
        return Some(IssueCategory::Security);
    }
    if has_automation {
        return Some(IssueCategory::Automation);
    }
    if has_testing {
        return Some(IssueCategory::Testing);
    }
    if has_docs {
        return Some(IssueCategory::Docs);
    }

    let soft_count = [has_bug, has_refactor, has_feature]
        .into_iter()
        .filter(|v| *v)
        .count();
    if soft_count >= 2 {
        return Some(IssueCategory::Mixed);
    }
    if has_bug {
        return Some(IssueCategory::BugFixes);
    }
    if has_refactor {
        return Some(IssueCategory::Refactoring);
    }
    if has_feature {
        return Some(IssueCategory::Features);
    }
    None
}

fn classify_latent(title: &str, body: &str) -> (IssueCategory, f64) {
    let text = format!("{} {}", title, body).to_lowercase();
    let sec = score(
        &text,
        &["security", "vuln", "codeql", "cve", "hardening", "sast"],
        1.0,
    );
    let bug = score(
        &text,
        &["bug", "fix", "panic", "crash", "race", "flaky", "error"],
        1.0,
    );
    let rf = score(
        &text,
        &[
            "refactor",
            "cleanup",
            "deduplicate",
            "boilerplate",
            "rework",
        ],
        1.0,
    );
    let feat = score(
        &text,
        &["feature", "enhancement", "introduce", "implement", "add"],
        1.0,
    );
    let auto = score(
        &text,
        &["workflow", "automation", "script", "pipeline", "ci", "lint"],
        1.0,
    );
    let test = score(
        &text,
        &["test", "assertion", "fixtures", "stabilize", "flaky"],
        1.0,
    );
    let docs = score(
        &text,
        &["documentation", "docs", "readme", "guide", "translate"],
        1.0,
    );

    let mut candidates = [
        (IssueCategory::Security, sec),
        (IssueCategory::BugFixes, bug),
        (IssueCategory::Refactoring, rf),
        (IssueCategory::Features, feat),
        (IssueCategory::Automation, auto),
        (IssueCategory::Testing, test),
        (IssueCategory::Docs, docs),
    ];

    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let top = candidates[0].clone();
    let second = candidates[1].clone();

    if top.1 <= 0.0 {
        return (IssueCategory::Unknown, 0.0);
    }

    // Lower confidence if categories are close.
    let margin = (top.1 - second.1).max(0.0);
    let confidence = (0.5 + (margin / (top.1 + 1.0))).min(0.99);
    (top.0, confidence)
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}

fn score(text: &str, tokens: &[&str], weight: f64) -> f64 {
    let mut s = 0.0;
    for t in tokens {
        if text.contains(t) {
            s += weight;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_label_priority() {
        let input = IssueClassificationInput {
            labels: vec!["security".to_string(), "bug".to_string()],
            title: "fix panic".to_string(),
            body: String::new(),
        };
        let res = classify_issue(&input, 0.7);
        assert_eq!(res.category, IssueCategory::Security);
        assert_eq!(res.source, CategorySource::DeterministicLabels);
    }

    #[test]
    fn mixed_when_multiple_soft_labels() {
        let input = IssueClassificationInput {
            labels: vec!["bug".to_string(), "feature".to_string()],
            title: "mixed".to_string(),
            body: String::new(),
        };
        let res = classify_issue(&input, 0.7);
        assert_eq!(res.category, IssueCategory::Mixed);
    }

    #[test]
    fn latent_fallback_without_labels() {
        let input = IssueClassificationInput {
            labels: vec![],
            title: "fix race condition in async writer".to_string(),
            body: "panic and flaky failures".to_string(),
        };
        let res = classify_issue(&input, 0.6);
        assert_eq!(res.source, CategorySource::LatentHeuristic);
        assert_eq!(res.category, IssueCategory::BugFixes);
    }
}
