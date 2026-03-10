use std::collections::BTreeMap;
use std::process::Command;

use crate::pr::commands::pr_duplicate_actions_options::PrDuplicateActionsOptions;

pub(crate) fn run_duplicate_actions(opts: PrDuplicateActionsOptions) -> i32 {
    let mode = opts.mode.trim();
    if mode != "safe" && mode != "auto-close" {
        eprintln!("--mode must be safe or auto-close");
        return 2;
    }
    if opts.repo.trim().is_empty() {
        eprintln!("--repo is required");
        return 2;
    }

    let duplicate_targets = parse_duplicate_targets(&opts.text);
    if duplicate_targets.is_empty() {
        println!("Duplicate mode ({mode}): no duplicate declarations detected.");
        return 0;
    }

    let auto_close_allowed = mode != "auto-close" || opts.assume_yes;
    if mode == "auto-close" && !opts.assume_yes {
        eprintln!(
            "Warning: duplicate auto-close requested without --assume-yes; close action will be skipped."
        );
    }

    for (duplicate_issue_key, canonical_issue_key) in duplicate_targets {
        let duplicate_issue_number = duplicate_issue_key.trim_start_matches('#');
        let comment_body = if mode == "safe" {
            format!(
                "Potential duplicate detected by PR generation workflow: {duplicate_issue_key} may duplicate {canonical_issue_key}. Please review manually."
            )
        } else {
            format!("Duplicate of {canonical_issue_key}")
        };

        let comment_status = gh_status(
            "api",
            &[
                &format!(
                    "repos/{}/issues/{}/comments",
                    opts.repo, duplicate_issue_number
                ),
                "-f",
                &format!("body={comment_body}"),
            ],
        );
        if comment_status != 0 {
            return comment_status;
        }
        println!(
            "Duplicate mode ({mode}): commented on {duplicate_issue_key} (target {canonical_issue_key})."
        );

        if mode == "auto-close" && auto_close_allowed {
            let close_status = gh_status(
                "api",
                &[
                    &format!("repos/{}/issues/{}", opts.repo, duplicate_issue_number),
                    "-X",
                    "PATCH",
                    "-f",
                    "state=closed",
                    "-f",
                    "state_reason=not_planned",
                ],
            );
            if close_status != 0 {
                return close_status;
            }
            println!("Duplicate mode ({mode}): closed {duplicate_issue_key}.");
        } else if mode == "auto-close" {
            println!(
                "Duplicate mode ({mode}): close skipped for {duplicate_issue_key} (missing --assume-yes)."
            );
        }
    }

    0
}

fn parse_duplicate_targets(text: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((duplicate_raw, canonical_raw)) = trimmed.split_once('|') else {
            continue;
        };
        let Some(duplicate_issue_key) = normalize_issue_key(duplicate_raw) else {
            continue;
        };
        let Some(canonical_issue_key) = normalize_issue_key(canonical_raw) else {
            continue;
        };
        out.insert(duplicate_issue_key, canonical_issue_key);
    }
    out
}

fn normalize_issue_key(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let digits = trimmed.strip_prefix('#')?;
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(format!("#{digits}"))
}

fn gh_status(cmd: &str, args: &[&str]) -> i32 {
    let mut command = Command::new("gh");
    command.arg(cmd);
    command.args(args);
    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute gh {}: {err}", cmd);
            1
        }
    }
}
