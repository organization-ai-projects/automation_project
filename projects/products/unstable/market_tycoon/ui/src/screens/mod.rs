//! projects/products/unstable/market_tycoon/ui/src/screens/mod.rs
mod replay_screen;
mod run_screen;
mod scenario_screen;

#[cfg(test)]
mod tests;

pub(crate) use replay_screen::ReplayScreen;
pub(crate) use run_screen::RunScreen;
pub(crate) use scenario_screen::ScenarioScreen;
