use serde::{Deserialize, Serialize};

use crate::fleets::fleet::Fleet;
use crate::map::star_system_id::StarSystemId;

/// Input data for a single battle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleInput {
    pub attacker: Fleet,
    pub defender: Fleet,
    pub location: StarSystemId,
}
