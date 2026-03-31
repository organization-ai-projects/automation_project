use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fact {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

impl Fact {
    pub fn new(
        subject: impl Into<String>,
        predicate: impl Into<String>,
        object: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    pub fn matches_pattern(
        &self,
        subject: Option<&str>,
        predicate: Option<&str>,
        object: Option<&str>,
    ) -> bool {
        let s_match = subject.map_or(true, |s| self.subject == s);
        let p_match = predicate.map_or(true, |p| self.predicate == p);
        let o_match = object.map_or(true, |o| self.object == o);
        s_match && p_match && o_match
    }
}
