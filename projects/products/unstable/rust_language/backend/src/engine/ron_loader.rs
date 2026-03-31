use crate::diagnostics::error::Error;
use crate::model::project_config::ProjectConfig;

use std::path::Path;

pub struct RonLoader;

impl RonLoader {
    pub fn load_config(path: &Path) -> Result<ProjectConfig, Error> {
        common_ron::read_ron(path).map_err(Error::from)
    }

    pub fn save_config(path: &Path, config: &ProjectConfig) -> Result<(), Error> {
        common_ron::write_ron(path, config).map_err(Error::from)
    }
}
