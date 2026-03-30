use serde::{Deserialize, Serialize};

use crate::plugins::plugin_id::PluginId;

/// A panel instance in the UI model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Panel {
    pub plugin_id: PluginId,
    pub title: String,
    pub content: Option<String>,
}

impl Panel {
    pub fn new(plugin_id: PluginId, title: String) -> Self {
        Self {
            plugin_id,
            title,
            content: None,
        }
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}
