pub mod feedback;
pub mod generation;
pub mod inference;
pub mod solver;
pub mod training;
pub mod network;
pub mod tokenization;

// Ré-exporte pour faciliter l'utilisation
pub use solver::NeuralSolver;
pub use solver::{NeuralError, SolverResult};

pub fn init() {
    println!("Initializing neural library...");
}
