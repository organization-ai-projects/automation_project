//! tools/versioning_automation/src/lazy_regex.rs
use regex::Regex;
use std::sync::LazyLock;

pub(crate) static _PARENT_FIELD_CAPTURE_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"(?im)^\\s*Parent\\s*:\\s*(.+)$"));

pub(crate) static _ISSUE_CLOSURE_REF_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"(?i)\\b(closes|fixes)\\b\\s+(rejected\\s+)?[^#\\s]*#([0-9]+)"));

pub(crate) static _ISSUE_REF_IN_SUBJECT_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| {
        Regex::new(
            r"(?i)(^|[[:space:]])(cancel[\\s_-]*closes|closes|part[[:space:]]+of|reopen|reopens|fixes)[[:space:]]+#[0-9]+([[:space:]]|$)",
        )
    });

pub(crate) static _GENERAL_ISSUE_REF_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| {
        Regex::new(
            r"(?i)(cancel[\\s_-]*closes|closes|fixes|part\\s+of|reopen|reopens)\\s+#([0-9]+)",
        )
    });

pub(crate) static PARENT_FIELD_REGEX: LazyLock<Result<Regex, regex::Error>> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*Parent:\s*(#?[0-9]+|none|base|epic|\(none\)|\(base\)|\(epic\))\s*$")
});

pub(crate) static COMMIT_MESSAGE_FORMAT_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| {
        Regex::new(
            r"^(feature|feat|fix|doc|docs|refactor|test|tests|chore|perf)(\([a-zA-Z0-9_./,-]+\))?:[[:space:]].+$",
        )
    });

pub(crate) static BRANCH_NAME_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^(feature|fix|hotfix|release)/[a-zA-Z0-9_-]+$"));

pub(crate) static SCOPE_EXTRACTION_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^[a-z]+\(([^)]+)\):"));

pub(crate) static ISSUE_PREFIX_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z]+-[0-9]+[-_/]"));

pub(crate) static SEMVER_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^[0-9]+\\.[0-9]+\\.[0-9]+(-[a-zA-Z0-9.-]+)?$"));
