pub enum CliCommand {
    Plan { dsl_file: String, json: bool },
    Apply { plan_file: String },
    Verify { plan_file: String, json: bool },
    Serve,
}

pub fn parse_args(args: &[String]) -> Result<CliCommand, String> {
    if args.len() < 2 {
        return Err("usage: patchsmith_backend <plan|apply|verify|serve> [OPTIONS]".into());
    }
    match args[1].as_str() {
        "plan" => {
            let dsl_file = find_flag(args, "--dsl").ok_or("plan requires --dsl <file>")?;
            let json = args.iter().any(|a| a == "--json");
            Ok(CliCommand::Plan { dsl_file, json })
        }
        "apply" => {
            let plan_file = find_flag(args, "--plan").ok_or("apply requires --plan <file>")?;
            Ok(CliCommand::Apply { plan_file })
        }
        "verify" => {
            let plan_file = find_flag(args, "--plan").ok_or("verify requires --plan <file>")?;
            let json = args.iter().any(|a| a == "--json");
            Ok(CliCommand::Verify { plan_file, json })
        }
        "serve" => Ok(CliCommand::Serve),
        other => Err(format!("unknown command: {other}")),
    }
}

fn find_flag(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
}
