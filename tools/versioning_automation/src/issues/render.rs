//! tools/versioning_automation/src/issues/render.rs
use crate::issues::model::CreateOptions;

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
    body
}

pub(crate) fn print_usage() {
    println!("Usage:");
    println!(
        "  va issue create --title ... --context ... --problem ... --acceptance ... [--parent ...] [--label ...] [--repo ...] [--dry-run]"
    );
    println!(
        "  va issue read [--issue <number>] [--repo owner/name] [--json fields] [--jq filter] [--template tpl]"
    );
    println!(
        "  va issue update --issue <number> [--repo owner/name] [--title ...|--body ...|--add-label ...|--remove-label ...|--add-assignee ...|--remove-assignee ...]"
    );
    println!(
        "  va issue close --issue <number> [--repo owner/name] [--reason completed|not_planned]"
    );
    println!("  va issue reopen --issue <number> [--repo owner/name]");
    println!("  va issue delete --issue <number> [--repo owner/name]");
}
