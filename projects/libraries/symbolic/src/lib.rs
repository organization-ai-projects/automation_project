pub mod analyzer;
pub mod generation;
pub mod linter;
pub mod rules;
pub mod solver;
pub mod workflow;
pub mod validator;

// Ré-exporte pour faciliter l'utilisation
pub use solver::SymbolicSolver;
pub use solver::{SolverResult, SymbolicError, ValidationResult};

pub fn init() {
    println!("Initializing symbolic library...");
}
