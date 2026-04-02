#[path = "debate.rs"]
pub mod debate_model;
pub mod debate_resolver;

pub use debate_model::Debate;

#[cfg(test)]
mod tests;
