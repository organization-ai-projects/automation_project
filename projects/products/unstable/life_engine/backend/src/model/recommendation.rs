use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationOutput {
    pub actions: Vec<String>,
    pub estimations: Vec<String>,
    pub warnings: Vec<String>,
    pub opportunities: Vec<String>,
}
