use crate::diagnostics::error::SpaceDiploWarsError;
use crate::diplomacy::diplomacy_engine::DiplomacyEngine;
use crate::economy::economy_engine::EconomyEngine;
use crate::queues::queue_engine::QueueEngine;
use crate::report::run_report::RunReport;
use crate::resolution::resolution_engine::ResolutionEngine;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::tech::tech_engine::TechEngine;
use crate::time::phase::Phase;

use super::replay_file::ReplayFile;

pub struct ReplayEngine;

impl ReplayEngine {
    /// Replay all orders from a ReplayFile, verifying SnapshotHash at checkpoints.
    /// Returns the RunReport produced by the replay.
    pub fn replay(replay: &ReplayFile) -> Result<RunReport, SpaceDiploWarsError> {
        let mut state = replay.scenario.build_initial_state();

        let turns = replay
            .orders_per_turn
            .keys()
            .filter_map(|k| k.parse::<u64>().ok())
            .max()
            .unwrap_or(0);

        let mut turn_reports = Vec::new();

        for turn in 1..=turns {
            for _ in 0..replay.ticks_per_turn {
                state.current_phase = Phase::EconomyTick;
                EconomyEngine::tick(&mut state);
                QueueEngine::tick(&mut state);
                TechEngine::tick(&mut state);
                state.current_tick = crate::time::tick::Tick(state.current_tick.0 + 1);
            }

            state.current_phase = Phase::OrdersSubmit;
            let key = turn.to_string();
            let mut orders = replay
                .orders_per_turn
                .get(&key)
                .map(|os| os.orders.clone())
                .unwrap_or_default();
            if let Some(decisions) = replay.treaty_decisions.get(&key) {
                DiplomacyEngine::inject_scripted_decisions(&state, turn, decisions, &mut orders);
            }

            state.current_phase = Phase::OrdersResolve;
            let res_report = ResolutionEngine::resolve_turn(&mut state, &orders, turn);
            state.current_phase = Phase::Aftermath;

            // Verify checkpoint if present
            for cp in &replay.scenario.checkpoints {
                if cp.turn == turn
                    && let Some(expected_hash) = &cp.expected_snapshot_hash
                {
                    let snapshot = StateSnapshot::from_state(&state);
                    let computed = SnapshotHash::compute(&snapshot)?;
                    if computed.0 != *expected_hash {
                        return Err(SpaceDiploWarsError::ReplayMismatch(format!(
                            "Snapshot hash mismatch at turn {}: expected {}, got {}",
                            turn, expected_hash, computed.0
                        )));
                    }
                }
            }

            turn_reports.push(crate::report::turn_report::TurnReport {
                turn,
                battles: res_report.battles.clone(),
                diplomacy_events: res_report.diplomacy_events.clone(),
                validation_errors: res_report.validation_errors.clone(),
            });

            state.current_turn = crate::time::turn::Turn(turn);
        }

        let final_snapshot = StateSnapshot::from_state(&state);
        let snapshot_hash = SnapshotHash::compute(&final_snapshot)?;
        let treaty_reports = RunReport::treaty_reports_from_treaties(&state.treaties);
        let war_report = RunReport::war_report_from_turn_reports(&turn_reports);

        Ok(RunReport {
            game_id: state.game_id.0.clone(),
            seed: replay.seed,
            turns_played: turns,
            turn_reports,
            treaty_reports,
            war_report,
            final_snapshot_hash: snapshot_hash.0,
        })
    }

    pub fn snapshot_at_turn(
        replay: &ReplayFile,
        turn_limit: u64,
    ) -> Result<StateSnapshot, SpaceDiploWarsError> {
        let mut state = replay.scenario.build_initial_state();

        for turn in 1..=turn_limit {
            for _ in 0..replay.ticks_per_turn {
                state.current_phase = Phase::EconomyTick;
                EconomyEngine::tick(&mut state);
                QueueEngine::tick(&mut state);
                TechEngine::tick(&mut state);
                state.current_tick = crate::time::tick::Tick(state.current_tick.0 + 1);
            }

            state.current_phase = Phase::OrdersSubmit;
            let key = turn.to_string();
            let mut orders = replay
                .orders_per_turn
                .get(&key)
                .map(|os| os.orders.clone())
                .unwrap_or_default();
            if let Some(decisions) = replay.treaty_decisions.get(&key) {
                DiplomacyEngine::inject_scripted_decisions(&state, turn, decisions, &mut orders);
            }

            state.current_phase = Phase::OrdersResolve;
            ResolutionEngine::resolve_turn(&mut state, &orders, turn);
            state.current_phase = Phase::Aftermath;
            state.current_turn = crate::time::turn::Turn(turn);
        }

        Ok(StateSnapshot::from_state(&state))
    }
}
