pub mod analyzer;
pub mod feedback_symbolic;
pub mod generation;
pub mod linter;
pub mod rules;
pub mod solver_result;
pub mod symbolic_error;
pub mod symbolic_solver;
pub mod validation_result;
pub mod validator;
pub mod workflow;

// Ré-exporte pour faciliter l'utilisation
pub use symbolic_error::SymbolicError;

pub fn init() {
    println!("Initializing symbolic library...");
}
