use crate::controller::Controller;
use crate::diagnostics::tactics_grid_error::TacticsGridError;

pub fn run(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        print_usage();
        return 2;
    }

    let result = match args[1].as_str() {
        "run" => Controller::run(&args[2..]),
        "replay" => Controller::replay(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            return 2;
        }
    };

    match result {
        Ok(message) => {
            println!("{message}");
            0
        }
        Err(error) => {
            eprintln!("Error: {error}");
            match error {
                TacticsGridError::Io(_) | TacticsGridError::Json(_) => 3,
                TacticsGridError::Battle(_) => 4,
                TacticsGridError::ReplayMismatch(_) => 5,
                TacticsGridError::InvalidScenario(_) => 2,
            }
        }
    }
}

fn print_usage() {
    println!("tactics_grid - deterministic grid tactics engine");
    println!();
    println!("Commands:");
    println!("  run --seed S --scenario <name|file> --out <path> [--replay-out <path>]");
    println!("  replay --replay <path> --out <path>");
}
