//! tools/versioning_automation/src/cli_action.rs
pub enum CliAction {
    ShowHelp(String),
    RunAutomation(Vec<String>),
    RunGit(Vec<String>),
    RunPr(Vec<String>),
    RunIssue(Vec<String>),
}

impl CliAction {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        if args.len() <= 1 {
            return Ok(Self::ShowHelp(Self::help_text()));
        }

        match args[1].as_str() {
            "help" | "--help" | "-h" => Ok(Self::ShowHelp(Self::help_text())),
            "automation" => Ok(Self::RunAutomation(args[2..].to_vec())),
            "git" => Ok(Self::RunGit(args[2..].to_vec())),
            "pr" => Ok(Self::RunPr(args[2..].to_vec())),
            "issue" => Ok(Self::RunIssue(args[2..].to_vec())),
            unknown => Err(format!(
                "Unknown subcommand: {unknown}\n\n{}",
                Self::help_text()
            )),
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
            "  automation  Run versioning automation helper flows",
            "  git      Run git/versioning automation flow",
            "  pr       Run PR automation engine flow",
            "  issue    Run issue automation engine flow",
            "  help     Show this help",
        ];
        lines.join("\n")
    }
}
