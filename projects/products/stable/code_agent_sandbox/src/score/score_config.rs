/// Small "symbolic" scoring layer: tries to estimate code quality signals
/// from outputs (cargo/clippy) and from file contents in results (if present).
///
/// This is NOT perfect. It's intentionally simple and deterministic.
/// The goal is to shape rewards and guide the agent.
#[derive(Debug, Clone)]
pub(crate) struct ScoreConfig {
    pub(crate) penalize_unwrap_outside_tests: bool,
    pub(crate) unwrap_penalty: i32,
    pub(crate) penalize_panic_outside_tests: bool,
    pub(crate) panic_penalty: i32,
    pub(crate) penalize_dbg_outside_tests: bool,
    pub(crate) dbg_penalty: i32,
}
