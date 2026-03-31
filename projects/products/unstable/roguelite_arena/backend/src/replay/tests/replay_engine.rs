use crate::combat::CombatEngine;
use crate::diagnostics::RogueliteArenaError;
use crate::replay::ReplayEngine;
use crate::replay::ReplayFile;
use crate::rng::Seed;
use crate::scenarios::ScenarioLoader;

#[test]
fn replay_matches_run_hash_for_same_draws() -> Result<(), String> {
    let scenario = ScenarioLoader::default_scenario("arena_basic");
    let (run_report, draws) = CombatEngine::run(&scenario, 20, 42).map_err(|e| e.to_string())?;

    let replay = ReplayFile {
        seed: Seed(42),
        ticks: 20,
        scenario_name: "arena_basic".to_string(),
        rng_draws: draws,
    };
    let replay_report = ReplayEngine::replay(&replay).map_err(|e| e.to_string())?;
    if replay_report.run_hash.0 != run_report.run_hash.0 {
        return Err("replay hash does not match run hash".to_string());
    }
    Ok(())
}

#[test]
fn replay_detects_rng_mismatch() -> Result<(), String> {
    let scenario = ScenarioLoader::default_scenario("arena_basic");
    let (_, mut draws) = CombatEngine::run(&scenario, 20, 7).map_err(|e| e.to_string())?;
    let first = draws
        .first_mut()
        .ok_or_else(|| "expected at least one RNG draw".to_string())?;
    first.raw_value = first.raw_value.wrapping_add(1);

    let replay = ReplayFile {
        seed: Seed(7),
        ticks: 20,
        scenario_name: "arena_basic".to_string(),
        rng_draws: draws,
    };

    match ReplayEngine::replay(&replay) {
        Err(RogueliteArenaError::ReplayMismatch(_)) => Ok(()),
        Err(other) => Err(format!("unexpected error kind: {other}")),
        Ok(_) => Err("expected replay mismatch but got success".to_string()),
    }
}
