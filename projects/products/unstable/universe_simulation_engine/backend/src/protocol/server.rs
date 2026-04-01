use crate::controller::run_controller::RunController;
use crate::controller::save_controller::SaveController;
use crate::diagnostics::engine_error::EngineError;

pub fn run(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        print_usage();
        return 2;
    }

    let result = match args[1].as_str() {
        "run" => RunController::run(&args[2..]),
        "convert" => SaveController::load_and_convert(&args[2..]),
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
                EngineError::Io(_) | EngineError::Json(_) => 3,
                EngineError::Sim(_) | EngineError::InvalidConfig(_) => 4,
                EngineError::BinaryCodec(_) | EngineError::RonCodec(_) => 5,
            }
        }
    }
}

fn print_usage() {
    println!("universe_simulation_engine - deterministic universe simulation");
    println!();
    println!("Commands:");
    println!("  run --ticks N --seed S --ticks-per-era T --out <path> [--save-bin <path>] [--save-ron <path>] [--no-gravity] ...");
    println!("  convert --input-bin <path> --output-ron <path>");
}
