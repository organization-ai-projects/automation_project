// projects/products/unstable/autonomy_orchestrator_ai/src/configs/config_io_plan.rs

use crate::configs::{ConfigLoadMode, ConfigSaveMode};

#[derive(Clone, Debug, Default)]
pub struct ConfigIoPlan {
    pub load: Option<ConfigLoadMode>,
    pub saves: Vec<ConfigSaveMode>,
}
