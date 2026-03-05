use serde::{Deserialize, Serialize};

use super::star_system_id::StarSystemId;

/// A route between two star systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub from: StarSystemId,
    pub to: StarSystemId,
    pub distance: u32,
}
