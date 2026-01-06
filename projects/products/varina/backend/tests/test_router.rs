use backend::git_github::handlers::{ApplyRequest, PreviewRequest};
use backend::protocol::{Command, CommandResponse};
use backend::router::handle_command;

#[test]
fn test_handle_preview_command() {
    let cmd = Command::PreviewGitAutopilot(PreviewRequest {
        policy_overrides: None,
    });

    let response = handle_command(cmd).expect("La commande PreviewGitAutopilot devrait réussir");

    if let CommandResponse::PreviewGitAutopilot(res) = response {
        assert!(
            res.report.logs.is_empty(),
            "Les logs devraient être vides par défaut"
        );
    } else {
        panic!("Réponse inattendue pour PreviewGitAutopilot");
    }
}

#[test]
fn test_handle_apply_command() {
    let cmd = Command::ApplyGitAutopilot(ApplyRequest {
        policy_overrides: None,
    });

    let response = handle_command(cmd).expect("La commande ApplyGitAutopilot devrait réussir");

    if let CommandResponse::ApplyGitAutopilot(res) = response {
        assert!(
            res.report.logs.is_empty(),
            "Les logs devraient être vides par défaut"
        );
    } else {
        panic!("Réponse inattendue pour ApplyGitAutopilot");
    }
}
