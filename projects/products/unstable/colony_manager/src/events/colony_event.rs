use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColonyEvent {
    Raid { severity: u32 },
    Sickness { colonist_name: String },
    Traders { goods: Vec<String> },
    Windfall { resource: String, amount: u32 },
}
