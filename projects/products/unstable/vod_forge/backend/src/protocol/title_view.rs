// projects/products/unstable/vod_forge/backend/src/protocol/title_view.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleView {
    pub id: String,
    pub name: String,
    pub year: u16,
    pub episode_count: usize,
}
