//projects/products/unstable/autonomous_dev_ai/src/tools/constants.rs
// Allowed git subcommands (allowlist, deny-by-default for all others).
pub const GIT_ALLOWED_SUBCOMMANDS: &[&str] = &[
    "status",
    "diff",
    "log",
    "show",
    "add",
    "commit",
    "checkout",
    "branch",
    "fetch",
    "stash",
    "rev-parse",
];

/// Default timeout for tool execution (seconds).
pub const DEFAULT_TOOL_TIMEOUT_SECS: u64 = 30;
