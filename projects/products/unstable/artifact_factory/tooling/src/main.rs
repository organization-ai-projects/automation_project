mod diagnostics;
mod public_api;
mod validate;

use crate::validate::bundle_validator::BundleValidator;
use crate::validate::hash_validator::HashValidator;
use std::path::Path;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("artifact-factory-tooling starting");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "validate" => {
            if args.len() < 3 {
                eprintln!("Usage: validate <bundle_dir> [--hash <expected_hash>]");
                std::process::exit(1);
            }
            let bundle_dir = Path::new(&args[2]);
            let expected_hash = args
                .iter()
                .position(|a| a == "--hash")
                .and_then(|i| args.get(i + 1))
                .cloned();

            // Validate presence of standard bundle files
            let expected_files = ["docs.md", "graph.svg", "docs.html"];
            match BundleValidator::validate(bundle_dir, &expected_files) {
                Ok(()) => tracing::info!("bundle structure valid"),
                Err(e) => {
                    eprintln!("Validation failed: {e}");
                    std::process::exit(1);
                }
            }

            // Hash verification if requested
            if let Some(hash) = expected_hash {
                match HashValidator::verify(bundle_dir, &hash) {
                    Ok(()) => tracing::info!("hash verified"),
                    Err(e) => {
                        eprintln!("Hash verification failed: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                match HashValidator::compute_hash(bundle_dir) {
                    Ok(h) => println!("Bundle hash: {h}"),
                    Err(e) => {
                        eprintln!("Hash computation failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
        }
        "hash" => {
            if args.len() < 3 {
                eprintln!("Usage: hash <bundle_dir>");
                std::process::exit(1);
            }
            let bundle_dir = Path::new(&args[2]);
            match HashValidator::compute_hash(bundle_dir) {
                Ok(h) => println!("{h}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("artifact-factory-tooling — bundle validation CLI");
    println!();
    println!("Commands:");
    println!("  validate <bundle_dir> [--hash <expected_hash>]");
    println!("  hash <bundle_dir>");
}
