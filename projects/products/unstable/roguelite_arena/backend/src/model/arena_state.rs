use crate::loot::Item;
use crate::model::Player;
use crate::model::Wave;
use crate::time::TickClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ArenaState {
    pub(crate) player: Player,
    pub(crate) current_wave: Option<Wave>,
    pub(crate) wave_index: u32,
    pub(crate) waves_cleared: u32,
    pub(crate) enemies_killed: u32,
    pub(crate) loot_collected: Vec<Item>,
    pub(crate) clock: TickClock,
}
