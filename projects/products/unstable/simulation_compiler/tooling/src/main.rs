// projects/products/unstable/simulation_compiler/tooling/src/main.rs
mod diagnostics;
mod public_api;
mod validate;

use diagnostics::error::ToolingError;

fn main() -> Result<(), ToolingError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "validate-pack" => run_validate_pack(&args),
        "check-goldens" => run_check_goldens(&args),
        _ => {
            eprintln!("error: unknown command '{}'", args[1]);
            print_usage();
            Err(ToolingError::Cli(format!("unknown command: {}", args[1])))
        }
    }
}

fn run_validate_pack(args: &[String]) -> Result<(), ToolingError> {
    let pack_dir = args.get(2).cloned().unwrap_or_default();
    tracing::info!(pack_dir = %pack_dir, "validate-pack started");

    let validator = validate::emitted_pack_validator::EmittedPackValidator::new();
    let result = validator.validate_dir(&pack_dir)?;
    tracing::info!(
        valid = result.valid,
        files = result.file_count,
        "validate-pack complete"
    );
    println!("valid={} files={}", result.valid, result.file_count);
    Ok(())
}

fn run_check_goldens(args: &[String]) -> Result<(), ToolingError> {
    let pack_dir = args.get(2).cloned().unwrap_or_default();
    let golden_dir = args.get(3).cloned().unwrap_or_default();
    tracing::info!(pack_dir = %pack_dir, golden_dir = %golden_dir, "check-goldens started");

    let validator = validate::golden_validator::GoldenValidator::new();
    let result = validator.check(&pack_dir, &golden_dir)?;
    tracing::info!(matched = result.matched, "check-goldens complete");
    println!("matched={}", result.matched);
    Ok(())
}

fn print_usage() {
    println!("simulation-compiler-tooling");
    println!();
    println!("Commands:");
    println!("  validate-pack <pack-dir>               Validate emitted pack structure");
    println!("  check-goldens <pack-dir> <golden-dir>  Compare pack output against goldens");
}
