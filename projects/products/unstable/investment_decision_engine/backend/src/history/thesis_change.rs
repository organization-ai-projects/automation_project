use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThesisDirection {
    Strengthened,
    Weakened,
    Broken,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThesisChange {
    pub date: String,
    pub direction: ThesisDirection,
    pub reason: String,
    pub prior_thesis: String,
    pub updated_thesis: Option<String>,
}

impl ThesisChange {
    pub fn new(
        date: impl Into<String>,
        direction: ThesisDirection,
        reason: impl Into<String>,
        prior_thesis: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            direction,
            reason: reason.into(),
            prior_thesis: prior_thesis.into(),
            updated_thesis: None,
        }
    }

    pub fn is_broken(&self) -> bool {
        self.direction == ThesisDirection::Broken
    }
}
