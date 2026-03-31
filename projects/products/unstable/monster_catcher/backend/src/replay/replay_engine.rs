use crate::diagnostics::error::BackendError;
use crate::events::event_log::EventLog;
use crate::model::inventory::Inventory;
use crate::model::party::Party;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn build_replay(
        seed: u64,
        step_count: u64,
        party: &Party,
        inventory: &Inventory,
        event_log: &EventLog,
    ) -> Result<String, BackendError> {
        let file = ReplayFile {
            seed,
            step_count,
            party: party.clone(),
            inventory: inventory.clone(),
            event_log: event_log.clone(),
        };
        ReplayCodec::encode(&file)
    }

    pub fn replay_to_end(
        raw: &str,
    ) -> Result<(Party, Inventory, EventLog, u64, u64), BackendError> {
        let file = ReplayCodec::decode(raw)?;
        Ok((
            file.party,
            file.inventory,
            file.event_log,
            file.step_count,
            file.seed,
        ))
    }
}
