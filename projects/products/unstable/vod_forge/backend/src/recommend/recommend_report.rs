use serde::{Deserialize, Serialize};
use crate::playback::profile_id::ProfileId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendReport {
    pub profile_id: ProfileId,
    pub recommended: Vec<String>,
}
