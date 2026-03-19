use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct MainPrRefSnapshot {
    #[serde(default, rename = "baseRefName")]
    pub(crate) base_ref_name: String,
    #[serde(default, rename = "headRefName")]
    pub(crate) head_ref_name: String,
}
