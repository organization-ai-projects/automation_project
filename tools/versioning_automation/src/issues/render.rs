//! tools/versioning_automation/src/issues/render.rs

pub(crate) fn print_usage() {
    println!("Usage:");
    println!(
        "  va issue create --title ... --context ... --problem ... --acceptance ... [--parent ...] [--label ...] [--assignee ...] [--related-issue ...] [--related-pr ...] [--repo ...] [--dry-run]"
    );
    println!(
        "  va issue read [--issue <number>] [--repo owner/name] [--json fields] [--jq filter] [--template tpl]"
    );
    println!(
        "  va issue done-status (--on-dev-merge --pr <number> | --on-issue-closed --issue <number>) [--label <name>] [--repo owner/name]"
    );
    println!("  va issue reopen-on-dev --pr <number> [--label <name>] [--repo owner/name]");
    println!(
        "  va issue update --issue <number> [--repo owner/name] [--title ...|--body ...|--add-label ...|--remove-label ...|--add-assignee ...|--remove-assignee ...]"
    );
    println!("  va issue repo-name");
    println!("  va issue current-login");
    println!("  va issue is-root-parent --issue <number> [--repo owner/name]");
    println!("  va issue validate-footer --file <path> [--repo owner/name]");
    println!(
        "  va issue close --issue <number> [--repo owner/name] [--reason completed|not_planned] [--comment ...]"
    );
    println!("  va issue reopen --issue <number> [--repo owner/name]");
    println!("  va issue delete --issue <number> [--repo owner/name]");
    println!("  va issue reevaluate --issue <number> [--repo owner/name]");
    println!("  va issue neutralize --pr <number> [--repo owner/name]");
    println!("  va issue auto-link --issue <number> [--repo owner/name]");
    println!(
        "  va issue parent-guard (--issue <number> | --child <number>) [--strict-guard true|false]"
    );
    println!("  va issue closure-hygiene [--repo owner/name]");
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
    println!("  va issue open-snapshots [--repo owner/name] [--limit <n>]");
    println!("  va issue extract-refs [--profile <hook|audit>] (--text <raw> | --file <path>)");
    println!("  va issue assignee-logins --issue <number> [--repo owner/name]");
    println!("  va issue state --issue <number> [--repo owner/name]");
    println!("  va issue has-label --issue <number> --label <name> [--repo owner/name]");
    println!("  va issue list-by-label --label <name> [--repo owner/name]");
    println!(
        "  va issue field --issue <number> --name <title|body|labels-raw> [--repo owner/name]"
    );
    println!(
        "  va issue upsert-marker-comment --repo owner/name --issue <number> --marker <marker> --body <body> [--announce true|false]"
    );
}
