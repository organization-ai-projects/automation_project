use crate::router::handle_command;
use protocol::{Command, CommandResponse};

pub fn run_backend() -> Result<(), String> {
    // Exemple de simulation de commandes pour debug (sous feature flag `debug_cli`)
    #[cfg(feature = "debug_cli")]
    {
        let cmd = Command::PreviewGitAutopilot(Default::default());
        let response = handle_command(cmd)?;
        println!("Réponse simulée : {:?}", response);
    }

    Ok(())
}
