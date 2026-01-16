// projects/products/core/launcher/src/build.rs
use crate::default_profile;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Build {
    #[serde(default = "default_build_enabled")]
    pub enabled: bool,
    #[serde(default = "default_profile")]
    pub profile: String, // "debug" | "release"
    #[serde(default)]
    pub extra_args: Vec<String>,
}

pub fn default_build_enabled() -> bool {
    true
}
