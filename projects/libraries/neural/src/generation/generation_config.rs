// projects/libraries/neural/src/generation/generation_config.rs

use common::CommonID;
use common::Id128;

#[derive(Debug, Clone)]
pub struct GenerationConfig {
    pub max_new_tokens: usize,
    pub temperature: f32,
    pub top_k: Option<usize>,
    pub top_p: Option<f32>,
    pub stop_token_id: usize,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            max_new_tokens: 512,
            temperature: 0.8,
            top_k: Some(50),
            top_p: Some(0.95),
            stop_token_id: 0,
        }
    }
}

impl GenerationConfig {
    pub fn validate_stop_token_id(&self) -> bool {
        CommonID::is_valid(Id128::new(self.stop_token_id as u16, None, None))
    }
}
