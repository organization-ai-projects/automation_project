use serde::{Deserialize, Serialize};
use crate::playback::profile_id::ProfileId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: ProfileId,
    pub name: String,
}
