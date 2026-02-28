use serde::{Deserialize, Serialize};

use super::star_system_id::StarSystemId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub id: StarSystemId,
    pub name: String,
    pub planets: Vec<String>,
    pub owner: Option<String>,
}
