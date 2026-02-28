mod diagnostics;
mod generate;
mod public_api;
mod validate;

use crate::public_api::{ContractValidator, PackGenerator};
use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    match args[1].as_str() {
        "generate-pack" => {
            let pack_name = args.get(2).cloned().unwrap_or_default();
            if pack_name.is_empty() {
                eprintln!("Usage: simkernel_tooling generate-pack <pack_name> [--out <dir>]");
                std::process::exit(2);
            }
            let out_dir = if let Some(pos) = args.iter().position(|a| a == "--out") {
                PathBuf::from(
                    args.get(pos + 1)
                        .cloned()
                        .unwrap_or_else(|| ".".to_string()),
                )
            } else {
                PathBuf::from(".")
            };
            match PackGenerator::generate(&pack_name, &out_dir) {
                Ok(_) => println!("Generated pack: {}", pack_name),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(5);
                }
            }
        }
        "validate-contract" => {
            let pack_file = args.get(2).cloned().unwrap_or_default();
            if pack_file.is_empty() {
                eprintln!("Usage: simkernel_tooling validate-contract --pack <file>");
                std::process::exit(2);
            }
            let path = PathBuf::from(&pack_file);
            match ContractValidator::validate(&path) {
                Ok(_) => println!("Contract valid"),
                Err(e) => {
                    eprintln!("Contract violation: {}", e);
                    std::process::exit(3);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(2);
        }
    }
}

fn print_usage() {
    println!("simkernel_tooling - SimKernel pack generator and validator");
    println!();
    println!("Commands:");
    println!("  generate-pack <pack_name> [--out <dir>]");
    println!("  validate-contract --pack <file>");
}
