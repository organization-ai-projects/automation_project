use crate::domain::contradiction_memory::ContradictionMemory;
use crate::domain::journal_event::JournalEvent;
use crate::domain::observation_slice::ObservationSlice;
use crate::domain::report_model::TickReport;
use crate::domain::tick_index::TickIndex;
use crate::domain::weather_state::WeatherState;
use crate::simulation::constraint_engine::ConstraintEngine;
use crate::simulation::contradiction_recorder::ContradictionRecorder;
use crate::simulation::correction_engine::CorrectionEngine;
use crate::simulation::raw_prediction_engine::RawPredictionEngine;
use crate::simulation::state_transition::StateTransition;

pub struct TickProcessor;

impl TickProcessor {
    pub fn process(
        tick: TickIndex,
        prior_state: &WeatherState,
        slice: &ObservationSlice,
        seed: u64,
        memory: &mut ContradictionMemory,
    ) -> (TickReport, Vec<JournalEvent>, WeatherState) {
        let mut events = Vec::new();

        events.push(JournalEvent::TickStarted { tick });

        events.push(JournalEvent::ObservationsLoaded {
            tick,
            records: slice.records.clone(),
        });

        let new_state = StateTransition::derive(prior_state, &slice.records);
        events.push(JournalEvent::StateUpdated {
            tick,
            state: new_state.clone(),
        });

        let raw_prediction = RawPredictionEngine::predict(&new_state, seed, tick.0);
        events.push(JournalEvent::RawPredictionGenerated {
            tick,
            prediction: raw_prediction.clone(),
        });

        let validation = ConstraintEngine::validate(&new_state, &raw_prediction);
        events.push(JournalEvent::ConstraintsEvaluated {
            tick,
            violations: validation.violations.clone(),
        });

        let correction_result = CorrectionEngine::correct(&raw_prediction, &validation);
        events.push(JournalEvent::CorrectionsApplied {
            tick,
            actions: correction_result.actions.clone(),
        });

        let corrected = correction_result.corrected.clone();
        events.push(JournalEvent::CorrectedPredictionEmitted {
            tick,
            prediction: corrected.clone(),
        });

        ContradictionRecorder::record(
            memory,
            tick,
            &raw_prediction,
            &validation.violations,
            &correction_result.actions,
            &corrected,
        );

        events.push(JournalEvent::TickCompleted { tick });

        let tick_report = TickReport {
            tick,
            weather_state: new_state.clone(),
            raw_prediction,
            violations: validation.violations,
            corrections: correction_result.actions,
            corrected_prediction: corrected,
        };

        (tick_report, events, new_state)
    }
}
