// projects/products/stable/core/launcher/src/build.rs
use serde::Deserialize;

use crate::defaults::default_profile;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Build {
    #[serde(default = "default_build_enabled")]
    pub(crate) enabled: bool,
    #[serde(default = "default_profile")]
    pub(crate) profile: String, // "debug" | "release"
    #[serde(default)]
    pub(crate) extra_args: Vec<String>,
}

pub(crate) fn default_build_enabled() -> bool {
    true
}
