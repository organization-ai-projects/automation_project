pub mod feedback;
pub mod generation;
pub mod inference;
pub mod network;
pub mod solver;
pub mod tokenization;
pub mod training;

// Ré-exporte pour faciliter l'utilisation
pub use solver::NeuralSolver;
pub use solver::{NeuralError, SolverResult};

pub fn init() {
    println!("Initializing neural library...");
}
