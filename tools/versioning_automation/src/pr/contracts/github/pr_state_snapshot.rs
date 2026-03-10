use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct PrStateSnapshot {
    #[serde(default, rename = "state")]
    pub(crate) state: String,
}
