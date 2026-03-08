use crate::config::runner::run_simulation;
use crate::diagnostics::diplo_sim_error::DiploSimError;

#[test]
fn run_simulation_rejects_zero_turns() {
    let result = run_simulation(0, 1, "unused.json", 2, "out.json", None);
    assert!(matches!(result, Err(DiploSimError::Config(message)) if message.contains("num_turns")));
}

#[test]
fn run_simulation_rejects_zero_players() {
    let result = run_simulation(1, 1, "unused.json", 0, "out.json", None);
    assert!(
        matches!(result, Err(DiploSimError::Config(message)) if message.contains("num_players"))
    );
}
