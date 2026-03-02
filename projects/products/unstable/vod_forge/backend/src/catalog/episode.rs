use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub season: u32,
    pub number: u32,
    pub name: String,
    pub duration_secs: u32,
}
