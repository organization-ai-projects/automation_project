//! projects/products/stable/code_agent_sandbox/backend/src/score/mod.rs
mod score_config;
mod score_summary;

pub(crate) use score_config::ScoreConfig;
pub(crate) use score_summary::ScoreSummary;

#[cfg(test)]
mod tests;
