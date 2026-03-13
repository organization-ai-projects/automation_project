//! tools/versioning_automation/src/git/render.rs
pub(crate) fn print_usage() {
    let lines = [
        "versioning_automation git",
        "",
        "Usage:",
        "  versioning_automation git <subcommand> [options]",
        "",
        "Subcommands:",
        "  create-branch [name] [--remote origin] [--base dev]",
        "  create-work-branch <type> <description> [--remote origin] [--base dev]",
        "  push-branch [--remote origin]",
        "  add-commit-push <message> [--no-verify] [--remote origin]",
        "  delete-branch <name> [--force] [--remote origin] [--base dev]",
        "  finish-branch [name] [--remote origin] [--base dev]",
        "  create-after-delete [--remote origin] [--base dev]",
        "  clean-local-gone [--remote origin]",
        "  clean-branches [--dry-run] [--remote origin] [--base dev]",
        "  cleanup-after-pr [--delete-only] [--remote origin] [--base dev]",
    ];
    println!("{}", lines.join("\n"));
}
