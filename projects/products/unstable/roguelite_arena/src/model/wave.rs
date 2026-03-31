use crate::model::Enemy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Wave {
    pub(crate) index: u32,
    pub(crate) enemies: Vec<Enemy>,
}

impl Wave {
    pub(crate) fn is_cleared(&self) -> bool {
        self.enemies.iter().all(|e| !e.is_alive())
    }

    pub(crate) fn alive_enemies(&self) -> Vec<&Enemy> {
        self.enemies.iter().filter(|e| e.is_alive()).collect()
    }
}
