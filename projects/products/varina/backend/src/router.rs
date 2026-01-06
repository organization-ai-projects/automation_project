use crate::git_github::handlers::{handle_apply_git_autopilot, handle_preview_git_autopilot};
use protocol::{Command, CommandResponse};

pub fn handle_command(cmd: Command) -> Result<CommandResponse, String> {
    match cmd {
        Command::PreviewGitAutopilot(req) => {
            handle_preview_git_autopilot(req).map(CommandResponse::PreviewGitAutopilot)
        }
        Command::ApplyGitAutopilot(req) => {
            handle_apply_git_autopilot(req).map(CommandResponse::ApplyGitAutopilot)
        }
    }
}
