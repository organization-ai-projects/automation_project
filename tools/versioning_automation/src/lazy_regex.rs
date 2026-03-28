use regex::Regex;
use std::sync::LazyLock;

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
