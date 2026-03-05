use crate::diagnostics::colony_manager_error::ColonyManagerError;
use crate::public_api::BackendApi;

pub fn run(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        print_usage();
        return 2;
    }

    let result = match args[1].as_str() {
        "run" => BackendApi::run(&args[2..]),
        "replay" => BackendApi::replay(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            return 2;
        }
    };

    match result {
        Ok(message) => {
            println!("{}", message);
            0
        }
        Err(error) => {
            eprintln!("Error: {error}");
            match error {
                ColonyManagerError::Io(_) | ColonyManagerError::Json(_) => 3,
                ColonyManagerError::Sim(_) => 4,
                ColonyManagerError::ReplayMismatch(_) => 5,
                ColonyManagerError::InvalidScenario(_) => 2,
            }
        }
    }
}

fn print_usage() {
    println!("colony_manager - deterministic colony simulation");
    println!();
    println!("Commands:");
    println!("  run --ticks N --seed S [--scenario <path>] --out <path> [--replay-out <path>]");
    println!("  replay --replay <path> --out <path>");
}
