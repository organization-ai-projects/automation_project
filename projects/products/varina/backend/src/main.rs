//! projects/products/varina/backend/src/main.rs
use std::process;

use backend::app::run_backend;

fn main() {
    if let Err(e) = run_backend() {
        eprintln!("fatal: {e}");
        process::exit(1);
    }
}
