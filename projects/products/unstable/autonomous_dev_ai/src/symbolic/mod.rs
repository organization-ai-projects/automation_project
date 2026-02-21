//! Symbolic control layer - authoritative decision maker.

pub mod issue_taxonomy;
pub mod planner;
pub mod policy;
pub mod validator;

mod neural_proposal;
mod symbolic_controller;
mod validation_result;

pub use neural_proposal::NeuralProposal;
pub use symbolic_controller::SymbolicController;
pub use validation_result::ValidationResult;
