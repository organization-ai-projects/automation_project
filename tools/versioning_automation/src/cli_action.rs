//! tools/versioning_automation/src/cli_action.rs
pub enum CliAction {
    ShowHelp(String),
    RunPr(Vec<String>),
    RunIssue(Vec<String>),
}

pub fn parse(args: &[String]) -> Result<CliAction, String> {
    if args.len() <= 1 {
        return Ok(CliAction::ShowHelp(help_text()));
    }

    match args[1].as_str() {
        "help" | "--help" | "-h" => Ok(CliAction::ShowHelp(help_text())),
        "pr" => Ok(CliAction::RunPr(args[2..].to_vec())),
        "issue" => Ok(CliAction::RunIssue(args[2..].to_vec())),
        unknown => Err(format!("Unknown subcommand: {unknown}\n\n{}", help_text())),
    }
}

fn help_text() -> String {
    let lines = [
        "versioning_automation (va)",
        "",
        "Usage:",
        "  va <subcommand> [args...]",
        "",
        "Subcommands:",
        "  pr       Run PR automation engine flow",
        "  issue    Run issue automation engine flow",
        "  help     Show this help",
    ];
    lines.join("\n")
}
