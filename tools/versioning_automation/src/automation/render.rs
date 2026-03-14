//! tools/versioning_automation/src/git/render.rs
pub(crate) fn print_usage() {
    let lines = [
        "versioning_automation automation",
        "",
        "Usage:",
        "  versioning_automation automation <subcommand> [options]",
        "",
        "Subcommands:",
        "  audit-security",
        "  build-accounts-ui",
        "  build-ui-bundles",
        "  build-and-check-ui-bundles",
        "  pre-add-review",
        "  test-coverage",
        "  changed-crates [<ref1>] [<ref2>] [--output-format paths|default]",
        "  check-merge-conflicts [--remote origin] [--base-branch dev]",
        "  check-dependencies [--skip-outdated] [--skip-unused]",
        "  clean-artifacts [--skip-node-modules]",
        "  check-priority-issues [--repo owner/name]",
        "  labels-sync [--labels-file .github/labels.json] [--prune]",
        "  ci-watch-pr [--pr <number>] [--poll-interval <seconds>] [--max-wait <seconds>]",
        "  sync-main-dev-ci [--remote origin] [--main main] [--dev dev] [--sync-branch sync/main-into-dev]",
    ];
    println!("{}", lines.join("\n"));
}
