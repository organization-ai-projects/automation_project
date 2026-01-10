// projects/products/varina/backend/src/main.rs
use backend::app::run_backend;

fn main() {
    if let Err(e) = run_backend() {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
}
