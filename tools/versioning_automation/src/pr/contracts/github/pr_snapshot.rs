use serde::Deserialize;

use crate::pr::contracts::github::pr_author::PrAuthor;

#[derive(Debug, Deserialize)]
pub(crate) struct PrSnapshot {
    #[serde(default, rename = "state")]
    pub(crate) state: String,
    #[serde(default, rename = "baseRefName")]
    pub(crate) base_ref_name: String,
    #[serde(default)]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) body: String,
    #[serde(default)]
    pub(crate) author: Option<PrAuthor>,
}
