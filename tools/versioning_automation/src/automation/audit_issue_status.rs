use std::collections::BTreeSet;
use std::fs;

use common_json::Json;

use crate::automation::commands::AuditIssueStatusOptions;
use crate::parent_field::extract_parent_field;
use crate::pr::text_payload::extract_effective_issue_ref_sets;
use crate::repo_name::resolve_repo_name;

use super::execute::{ensure_git_repo, run_gh_output, run_git_output_preserve};

type IssueRefSets = (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>);
type AuditIssueStatusSections<'a> = (
    &'a [String],
    &'a [String],
    &'a [String],
    &'a [String],
    &'a [String],
);

pub(crate) fn run_audit_issue_status(opts: AuditIssueStatusOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let repo = resolve_repo_name(opts.repo).map_err(|e| e.to_string())?;
    let range = format!("{}..{}", opts.base_ref, opts.head_ref);

    let open_issues_json = run_gh_output(&[
        "issue",
        "list",
        "--state",
        "open",
        "--limit",
        &opts.limit.to_string(),
        "--json",
        "number,title,url,body,labels,state",
        "-R",
        &repo,
    ])?;
    let open_issues = parse_json_array(&open_issues_json, "open issues JSON")?;
    let total_open = open_issues.len();

    let commit_messages = run_git_output_preserve(&["log", &range, "--format=%B"])?;
    let (closing_refs, reopen_refs, part_refs) = extract_issue_refs_from_text(&commit_messages)?;

    let mut would_close_items = Vec::new();
    let mut would_reopen_items = Vec::new();
    let mut part_only_items = Vec::new();
    let mut unreferenced_items = Vec::new();
    let mut done_in_dev_items = Vec::new();

    for issue in open_issues {
        let Some(obj) = issue.as_object() else {
            continue;
        };
        let number = object_u64(obj, "number");
        if number == 0 {
            continue;
        }
        let issue_id = number.to_string();
        let title = object_string(obj, "title");
        let url = object_string(obj, "url");
        let body = object_string(obj, "body");
        let parent = extract_parent_field(&body).unwrap_or_else(|| "(none)".to_string());

        let labels_csv = obj
            .get("labels")
            .and_then(Json::as_array)
            .map(|labels| {
                labels
                    .iter()
                    .filter_map(|label| label.as_object())
                    .map(|label_obj| object_string(label_obj, "name").to_lowercase())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default();

        let line = format!("- [#{issue_id}]({url}) {title} (parent: {parent})");
        if labels_csv.contains("done-in-dev") {
            done_in_dev_items.push(line);
        } else if closing_refs.contains(&issue_id) {
            would_close_items.push(line);
        } else if reopen_refs.contains(&issue_id) {
            would_reopen_items.push(line);
        } else if part_refs.contains(&issue_id) {
            part_only_items.push(line);
        } else {
            unreferenced_items.push(line);
        }
    }

    let report = render_issue_audit_report(
        &repo,
        &range,
        total_open,
        (
            &done_in_dev_items,
            &would_close_items,
            &would_reopen_items,
            &part_only_items,
            &unreferenced_items,
        ),
    );

    if let Some(output_file) = opts.output_file {
        fs::write(&output_file, &report)
            .map_err(|e| format!("Failed to write report to '{}': {e}", output_file))?;
        println!("Generated file: {output_file}");
    }
    print!("{report}");
    Ok(())
}

pub(crate) fn extract_issue_refs_from_text(text: &str) -> Result<IssueRefSets, String> {
    Ok(extract_effective_issue_ref_sets(text))
}

pub(crate) fn render_issue_audit_report(
    repo: &str,
    range: &str,
    total_open: usize,
    sections: AuditIssueStatusSections<'_>,
) -> String {
    let (
        done_in_dev_items,
        would_close_items,
        would_reopen_items,
        part_only_items,
        unreferenced_items,
    ) = sections;
    let mut out = Vec::new();
    out.push("# Issue Status Audit".to_string());
    out.push("".to_string());
    out.push(format!("- Repository: `{repo}`"));
    out.push(format!("- Range: `{range}`"));
    out.push("".to_string());
    out.push("## Summary".to_string());
    out.push("".to_string());
    out.push(format!("- Open issues fetched: {total_open}"));
    out.push(format!(
        "- Would close on merge: {}",
        would_close_items.len()
    ));
    out.push(format!(
        "- Would reopen from current refs: {}",
        would_reopen_items.len()
    ));
    out.push(format!(
        "- Done in dev (label): {}",
        done_in_dev_items.len()
    ));
    out.push(format!(
        "- Part-of-only (not closing): {}",
        part_only_items.len()
    ));
    out.push(format!(
        "- Unreferenced in range: {}",
        unreferenced_items.len()
    ));
    out.push("".to_string());
    out.push("## Done In Dev (Label)".to_string());
    out.push("".to_string());
    if done_in_dev_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(done_in_dev_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Would Close On Merge".to_string());
    out.push("".to_string());
    if would_close_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(would_close_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Would Reopen".to_string());
    out.push("".to_string());
    if would_reopen_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(would_reopen_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Part-Of Only".to_string());
    out.push("".to_string());
    if part_only_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(part_only_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Unreferenced".to_string());
    out.push("".to_string());
    if unreferenced_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(unreferenced_items.iter().cloned());
    }
    out.push("".to_string());
    out.join("\n")
}

fn parse_json_array(payload: &str, context: &str) -> Result<Vec<Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_array()
        .cloned()
        .ok_or_else(|| format!("Expected JSON array for {context}"))
}

fn object_u64(object: &std::collections::HashMap<String, Json>, key: &str) -> u64 {
    object.get(key).and_then(Json::as_u64).unwrap_or(0)
}

fn object_string(object: &std::collections::HashMap<String, Json>, key: &str) -> String {
    object
        .get(key)
        .and_then(Json::as_str)
        .unwrap_or_default()
        .to_string()
}
