use anyhow::{Result, anyhow};
use crate::rules::ruleset_loader;

pub fn run_cli(args: &[String]) -> Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: meta_determinism_guard_tooling <command> [options]");
        eprintln!("Commands: validate, list-rules");
        return Ok(());
    }

    match args[1].as_str() {
        "validate" => {
            let path = find_flag(args, "--ruleset")
                .ok_or_else(|| anyhow!("--ruleset required"))?;
            let ruleset = ruleset_loader::load_from_file(&path)?;
            println!("Ruleset '{}' is valid ({} rules).", ruleset.name, ruleset.rules.len());
        }
        "list-rules" => {
            let path = find_flag(args, "--ruleset")
                .ok_or_else(|| anyhow!("--ruleset required"))?;
            let ruleset = ruleset_loader::load_from_file(&path)?;
            println!("Rules in '{}':", ruleset.name);
            for rule in &ruleset.rules {
                println!("  - {} ({})", rule.name, rule.pattern);
            }
        }
        other => return Err(anyhow!("Unknown command: {}", other)),
    }

    Ok(())
}

fn find_flag(args: &[String], flag: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == flag)?;
    args.get(pos + 1).cloned()
}
