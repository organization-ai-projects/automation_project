use crate::time::Tick;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SimEvent {
    pub tick: Tick,
    pub kind: String,
    pub data: String,
}
