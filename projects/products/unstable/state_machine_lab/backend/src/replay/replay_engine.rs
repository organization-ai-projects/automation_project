use crate::diagnostics::error::BackendError;
use crate::execute::runner::Runner;
use crate::model::event_id::EventId;
use crate::model::machine::Machine;
use crate::replay::transcript::Transcript;
use crate::replay::transcript_codec::TranscriptCodec;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn build_transcript(
        machine: &Machine,
        events: &[EventId],
        seed: Option<u64>,
    ) -> Result<Transcript, BackendError> {
        let mut runner = Runner::new(machine.clone());
        let steps = runner.run_events(events)?;
        Ok(Transcript {
            machine_id: machine.id.clone(),
            seed,
            steps,
            final_state: runner.current_state.0.clone(),
        })
    }

    pub fn replay_transcript(
        machine: &Machine,
        transcript_raw: &str,
    ) -> Result<Transcript, BackendError> {
        let transcript = TranscriptCodec::decode(transcript_raw)?;
        let events: Vec<EventId> = transcript.steps.iter().map(|s| s.event.clone()).collect();
        let replayed = Self::build_transcript(machine, &events, transcript.seed)?;
        Ok(replayed)
    }

    pub fn verify_golden(machine: &Machine, golden_raw: &str) -> Result<bool, BackendError> {
        let golden = TranscriptCodec::decode(golden_raw)?;
        let golden_encoded = TranscriptCodec::encode(&golden)?;
        let events: Vec<EventId> = golden.steps.iter().map(|s| s.event.clone()).collect();
        let replayed = Self::build_transcript(machine, &events, golden.seed)?;
        let replayed_encoded = TranscriptCodec::encode(&replayed)?;
        Ok(golden_encoded == replayed_encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::machine::{Machine, Transition};
    use crate::model::machine_id::MachineId;
    use crate::model::state_id::StateId;
    use std::collections::BTreeMap;

    fn toggle_machine() -> Machine {
        let mut transitions = BTreeMap::new();
        transitions.insert(
            Machine::transition_key(&StateId("off".into()), &EventId("flip".into())),
            vec![Transition {
                event: EventId("flip".into()),
                target: StateId("on".into()),
                guard: None,
                action: None,
            }],
        );
        transitions.insert(
            Machine::transition_key(&StateId("on".into()), &EventId("flip".into())),
            vec![Transition {
                event: EventId("flip".into()),
                target: StateId("off".into()),
                guard: None,
                action: None,
            }],
        );
        Machine {
            id: MachineId("toggle".into()),
            initial_state: StateId("off".into()),
            states: vec![StateId("off".into()), StateId("on".into())],
            events: vec![EventId("flip".into())],
            transitions,
            variables: BTreeMap::new(),
        }
    }

    #[test]
    fn build_and_verify_golden() {
        let m = toggle_machine();
        let events = vec![EventId("flip".into()), EventId("flip".into())];
        let transcript = ReplayEngine::build_transcript(&m, &events, Some(1)).unwrap();
        let encoded = TranscriptCodec::encode(&transcript).unwrap();
        assert!(ReplayEngine::verify_golden(&m, &encoded).unwrap());
    }

    #[test]
    fn replay_matches_original() {
        let m = toggle_machine();
        let events = vec![
            EventId("flip".into()),
            EventId("flip".into()),
            EventId("flip".into()),
        ];
        let original = ReplayEngine::build_transcript(&m, &events, Some(42)).unwrap();
        let encoded = TranscriptCodec::encode(&original).unwrap();
        let replayed = ReplayEngine::replay_transcript(&m, &encoded).unwrap();
        assert_eq!(original, replayed);
    }
}
