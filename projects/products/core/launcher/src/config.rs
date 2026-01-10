// projects/products/core/launcher/src/config.rs
use serde::Deserialize;

use crate::{Build, Service, launcher::Launcher, workspace::Workspace};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workspace: Workspace,
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub launcher: Launcher,
    #[serde(default)]
    pub service: Vec<Service>,
}
