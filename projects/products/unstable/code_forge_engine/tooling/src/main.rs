// projects/products/unstable/code_forge_engine/tooling/src/main.rs
mod diagnostics;
mod golden;
mod public_api;
mod validate;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    run_cli(&args)
}

fn run_cli(args: &[String]) -> Result<()> {
    use golden::golden_updater::GoldenUpdater;
    use golden::golden_report::GoldenReport;
    use validate::byte_stability_validator::ByteStabilityValidator;
    use validate::structure_validator::StructureValidator;

    let sub = args.get(1).map(|s| s.as_str()).unwrap_or("");
    match sub {
        "update-goldens" => {
            let dir = args.get(2).map(|s| s.as_str()).unwrap_or("goldens");
            let updater = GoldenUpdater::new(dir);
            updater.update()?;
        }
        "validate" => {
            let dir = args.get(2).map(|s| s.as_str()).unwrap_or("goldens");
            let report = GoldenReport::new(dir);
            let result = report.check()?;
            if !result.all_passed {
                eprintln!("golden validation failed: {} failures", result.failures.len());
                std::process::exit(3);
            }
            let validator = ByteStabilityValidator::new(dir);
            validator.validate()?;
            let struct_validator = StructureValidator::new(dir);
            struct_validator.validate()?;
        }
        _ => {
            eprintln!("Usage: code_forge_engine_tooling <update-goldens|validate> [dir]");
            std::process::exit(2);
        }
    }
    Ok(())
}
