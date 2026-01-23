// projects/libraries/neural/src/lib.rs
pub mod feedback;
pub mod generation;
pub mod inference;
pub mod network;
pub mod solver;
pub mod tokenization;
pub mod training;

pub use solver::NeuralSolver;
pub use solver::{NeuralError, SolverResult};
