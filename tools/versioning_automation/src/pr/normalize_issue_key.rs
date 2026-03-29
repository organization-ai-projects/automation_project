//! tools/versioning_automation/src/pr/normalize_issue_key.rs
use regex::Regex;

use crate::pr::commands::PrNormalizeIssueKeyOptions;

pub(crate) fn run_normalize_issue_key(opts: PrNormalizeIssueKeyOptions) -> i32 {
    if let Some(issue_key) = normalize_issue_key(&opts.raw) {
        println!("{issue_key}");
        0
    } else {
        1
    }
}

fn normalize_issue_key(raw: &str) -> Option<String> {
    let re = Regex::new(r"#([0-9]+)").expect("valid regex");
    let captures = re.captures(raw)?;
    let digits = captures.get(1)?.as_str();
    Some(format!("#{digits}"))
}
