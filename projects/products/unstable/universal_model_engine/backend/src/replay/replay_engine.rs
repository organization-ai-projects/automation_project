use crate::diagnostics::backend_error::BackendError;
use crate::events::event_log::EventLog;
use crate::model::state::State;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn build_replay(
        seed: u64,
        step_count: u64,
        event_log: &EventLog,
        state: &State,
    ) -> Result<String, BackendError> {
        let file = ReplayFile {
            seed,
            step_count,
            state: state.clone(),
            event_log: event_log.clone(),
        };
        ReplayCodec::encode(&file)
    }

    pub fn replay_to_end(raw: &str) -> Result<(State, EventLog, u64), BackendError> {
        let file = ReplayCodec::decode(raw)?;
        Ok((file.state, file.event_log, file.step_count))
    }
}
