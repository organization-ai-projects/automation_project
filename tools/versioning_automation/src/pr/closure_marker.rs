use regex::Regex;

use crate::pr::contracts::cli::pr_closure_marker_options::PrClosureMarkerOptions;

pub(crate) fn run_closure_marker(opts: PrClosureMarkerOptions) -> i32 {
    let result = match opts.mode.as_str() {
        "apply" => apply_marker(&opts.text, &opts.keyword_pattern, &opts.issue),
        "remove" => remove_marker(&opts.text, &opts.keyword_pattern, &opts.issue),
        _ => {
            eprintln!("--mode must be apply or remove");
            return 2;
        }
    };

    match result {
        Ok(text) => {
            print!("{text}");
            0
        }
        Err(err) => {
            eprintln!("{err}");
            2
        }
    }
}

pub(crate) fn apply_marker(
    text: &str,
    keyword_pattern: &str,
    issue: &str,
) -> Result<String, String> {
    let issue_pattern = regex::escape(issue);
    let pattern = format!(
        "(?i)\\b(?P<kw>(?:{}))\\b(?P<ws>\\s+)(?P<rej>rejected\\s+)?(?P<ref>[^\\s]*{})\\b",
        keyword_pattern, issue_pattern
    );
    let re = Regex::new(&pattern).map_err(|err| format!("invalid keyword pattern: {err}"))?;
    Ok(re
        .replace_all(text, |caps: &regex::Captures<'_>| {
            let kw = caps.name("kw").map_or("", |m| m.as_str());
            let ws = caps.name("ws").map_or(" ", |m| m.as_str());
            let rej = caps.name("rej").map_or("", |m| m.as_str());
            let ref_part = caps.name("ref").map_or("", |m| m.as_str());
            if rej.is_empty() {
                format!("{kw}{ws}rejected {ref_part}")
            } else {
                format!("{kw}{ws}{rej}{ref_part}")
            }
        })
        .to_string())
}

pub(crate) fn remove_marker(
    text: &str,
    keyword_pattern: &str,
    issue: &str,
) -> Result<String, String> {
    let issue_pattern = regex::escape(issue);
    let pattern = format!(
        "(?i)\\b(?P<kw>(?:{}))\\b(?P<ws>\\s+)rejected\\s+(?P<ref>[^\\s]*{})\\b",
        keyword_pattern, issue_pattern
    );
    let re = Regex::new(&pattern).map_err(|err| format!("invalid keyword pattern: {err}"))?;
    Ok(re.replace_all(text, "${kw}${ws}${ref}").to_string())
}
