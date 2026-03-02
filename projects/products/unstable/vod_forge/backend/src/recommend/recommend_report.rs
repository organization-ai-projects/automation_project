use crate::playback::profile_id::ProfileId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendReport {
    pub profile_id: ProfileId,
    pub recommended: Vec<String>,
}
