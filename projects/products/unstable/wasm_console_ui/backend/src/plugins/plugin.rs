use serde::{Deserialize, Serialize};

use crate::plugins::plugin_id::PluginId;

/// A panel plugin definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plugin {
    id: PluginId,
    name: String,
}

impl Plugin {
    pub fn new(id: PluginId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &PluginId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
