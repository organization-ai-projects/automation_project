use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendRule {
    pub genre_match: bool,
    pub unwatched_only: bool,
}
