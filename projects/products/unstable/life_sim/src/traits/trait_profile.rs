use crate::traits::personality_trait::PersonalityTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitProfile {
    pub traits: Vec<PersonalityTrait>,
}

impl TraitProfile {
    pub fn new(traits: Vec<PersonalityTrait>) -> Self {
        let mut t = traits;
        t.truncate(5);
        Self { traits: t }
    }

    #[allow(dead_code)]
    pub fn has(&self, t: PersonalityTrait) -> bool {
        self.traits.contains(&t)
    }
}
