//! tools/versioning_automation/src/issues/render.rs
use crate::issues::commands::CreateOptions;

pub(crate) fn render_direct_issue_body(opts: &CreateOptions) -> String {
    let mut body = String::new();
    body.push_str("## Context\n\n");
    body.push_str(&opts.context);
    body.push_str("\n\n## Problem\n\n");
    body.push_str(&opts.problem);
    body.push_str("\n\n## Acceptance Criteria\n\nDone when :\n\n");
    for acceptance in &opts.acceptances {
        body.push_str("- [ ] ");
        body.push_str(acceptance);
        body.push('\n');
    }
    body.push_str("\n## Hierarchy\n\nParent: ");
    body.push_str(&opts.parent);

    let related_issues = opts
        .related_issues
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let related_prs = opts
        .related_prs
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    if !related_issues.is_empty() || !related_prs.is_empty() {
        body.push_str("\n\n## References\n");
        if !related_issues.is_empty() {
            body.push_str("\nRelated issue(s): ");
            body.push_str(&related_issues.join(" "));
        }
        if !related_prs.is_empty() {
            body.push_str("\nRelated PR(s): ");
            body.push_str(&related_prs.join(" "));
        }
    }

    body
}

pub(crate) fn print_usage() {
    println!("Usage:");
    println!(
        "  va issue create --title ... --context ... --problem ... --acceptance ... [--parent ...] [--label ...] [--assignee ...] [--related-issue ...] [--related-pr ...] [--repo ...] [--dry-run]"
    );
    println!(
        "  va issue read [--issue <number>] [--repo owner/name] [--json fields] [--jq filter] [--template tpl]"
    );
    println!(
        "  va issue update --issue <number> [--repo owner/name] [--title ...|--body ...|--add-label ...|--remove-label ...|--add-assignee ...|--remove-assignee ...]"
    );
    println!("  va issue repo-name");
    println!(
        "  va issue close --issue <number> [--repo owner/name] [--reason completed|not_planned] [--comment ...]"
    );
    println!("  va issue reopen --issue <number> [--repo owner/name]");
    println!("  va issue delete --issue <number> [--repo owner/name]");
    println!("  va issue reevaluate --issue <number> [--repo owner/name]");
    println!(
        "  va issue required-fields-validate [--mode title|body|content] [--title ...] [--body ...] [--labels-raw ...]"
    );
    println!("  va issue non-compliance-reason [--title ...] [--body ...] [--labels-raw ...]");
    println!("  va issue fetch-non-compliance-reason --issue <number> [--repo owner/name]");
    println!("  va issue label-exists --repo owner/name --label <name>");
    println!("  va issue sync-project-status --repo owner/name --issue <number> --status <name>");
    println!("  va issue tasklist-refs --body <issue_body>");
    println!("  va issue subissue-refs --owner <owner> --repo <repo> --issue <number>");
    println!("  va issue open-numbers [--repo owner/name]");
    println!("  va issue assignee-logins --issue <number> [--repo owner/name]");
    println!("  va issue state --issue <number> [--repo owner/name]");
    println!("  va issue has-label --issue <number> --label <name> [--repo owner/name]");
    println!(
        "  va issue upsert-marker-comment --repo owner/name --issue <number> --marker <marker> --body <body> [--announce true|false]"
    );
}
