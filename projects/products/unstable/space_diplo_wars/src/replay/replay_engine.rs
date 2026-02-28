use crate::diagnostics::error::SpaceDiploWarsError;
use crate::report::run_report::RunReport;
use crate::resolution::resolution_engine::ResolutionEngine;
use crate::scenario::scenario::Scenario;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;

use super::replay_file::ReplayFile;

pub struct ReplayEngine;

impl ReplayEngine {
    /// Replay all orders from a ReplayFile, verifying SnapshotHash at checkpoints.
    /// Returns the RunReport produced by the replay.
    pub fn replay(
        replay: &ReplayFile,
        scenario: &Scenario,
    ) -> Result<RunReport, SpaceDiploWarsError> {
        let mut state = scenario.build_initial_state();

        let turns = replay
            .orders_per_turn
            .keys()
            .filter_map(|k| k.parse::<u64>().ok())
            .max()
            .unwrap_or(0);

        let mut turn_reports = Vec::new();

        for turn in 1..=turns {
            let key = turn.to_string();
            let empty = Vec::new();
            let orders = replay
                .orders_per_turn
                .get(&key)
                .map(|os| os.orders.as_slice())
                .unwrap_or(empty.as_slice());

            let res_report = ResolutionEngine::resolve_turn(&mut state, orders, turn);

            // Verify checkpoint if present
            for cp in &scenario.checkpoints {
                if cp.turn == turn {
                    if let Some(expected_hash) = &cp.expected_snapshot_hash {
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
            }

            turn_reports.push(crate::report::turn_report::TurnReport {
                turn,
                battles: res_report.battles.clone(),
                diplomacy_events: res_report.diplomacy_events.clone(),
                validation_errors: res_report.validation_errors.clone(),
            });

            state.current_turn = crate::time::turn::Turn(turn);
            // Use ticks_per_turn=4 as default (same as run)
            state.current_tick = crate::time::tick::Tick(turn * 4);
        }

        let final_snapshot = StateSnapshot::from_state(&state);
        let snapshot_hash = SnapshotHash::compute(&final_snapshot)?;

        Ok(RunReport {
            game_id: state.game_id.0.clone(),
            seed: replay.seed,
            turns_played: turns,
            turn_reports,
            final_snapshot_hash: snapshot_hash.0,
        })
    }
}
