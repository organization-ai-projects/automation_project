use regex::Regex;
use std::sync::LazyLock;

pub(crate) static PARENT_FIELD_REGEX: LazyLock<Result<Regex, regex::Error>> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*Parent:\s*(#?[0-9]+|none|base|epic|\(none\)|\(base\)|\(epic\))\s*$")
});
