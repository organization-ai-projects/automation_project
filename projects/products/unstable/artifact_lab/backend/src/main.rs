mod bundle;
mod cli;
mod diagnostics;
mod hash;
mod manifest;
mod output;
mod verify;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = match cli::CliParser::parse(&args[1..]) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {e}");
            print_usage();
            process::exit(2);
        }
    };

    let result = match command {
        cli::CliCommand::Pack { root, out } => run_pack(&root, &out),
        cli::CliCommand::Unpack { bundle, out } => run_unpack(&bundle, &out),
        cli::CliCommand::Verify { bundle, json } => run_verify(&bundle, json),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(e.exit_code());
    }
}

fn run_pack(root: &str, out: &str) -> Result<(), diagnostics::Error> {
    let root_path = Path::new(root);
    let out_path = Path::new(out);

    if !root_path.is_dir() {
        return Err(diagnostics::Error::InvalidUsage(format!(
            "--root '{root}' is not a directory"
        )));
    }

    bundle::BundlePacker::pack(root_path, out_path)?;
    eprintln!("Packed '{root}' -> '{out}'");
    Ok(())
}

fn run_unpack(bundle: &str, out: &str) -> Result<(), diagnostics::Error> {
    let bundle_path = Path::new(bundle);
    let out_path = Path::new(out);

    if !bundle_path.is_file() {
        return Err(diagnostics::Error::InvalidUsage(format!(
            "--bundle '{bundle}' does not exist"
        )));
    }

    bundle::BundleUnpacker::unpack(bundle_path, out_path)?;
    eprintln!("Unpacked '{bundle}' -> '{out}'");
    Ok(())
}

fn run_verify(bundle: &str, json_output: bool) -> Result<(), diagnostics::Error> {
    let bundle_path = Path::new(bundle);

    if !bundle_path.is_file() {
        return Err(diagnostics::Error::InvalidUsage(format!(
            "--bundle '{bundle}' does not exist"
        )));
    }

    let report: verify::VerifyReport = verify::Verifier::verify(bundle_path)?;

    if json_output {
        println!("{}", output::canonical_json::render_verify_report(&report));
    } else {
        println!("bundle: {bundle}");
        println!("entries: {}", report.entry_count);
        for r in &report.results as &[verify::EntryResult] {
            println!("  {} [{}]", r.path, r.status_label());
        }
        println!("ok: {}", report.ok);
    }

    if !report.ok {
        process::exit(1);
    }

    Ok(())
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  artifact_lab pack   --root <dir> --out <bundle>");
    eprintln!("  artifact_lab unpack --bundle <bundle> --out <dir>");
    eprintln!("  artifact_lab verify --bundle <bundle> [--json]");
}
