// projects/products/stable/core/launcher/src/config.rs
use serde::Deserialize;

use crate::{build::Build, launcher::Launcher, service::Service, workspace::Workspace};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) workspace: Workspace,
    #[serde(default)]
    pub(crate) build: Build,
    #[serde(default)]
    pub(crate) launcher: Launcher,
    #[serde(default)]
    pub(crate) service: Vec<Service>,
}
