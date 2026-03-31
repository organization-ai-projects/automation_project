use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateValue {
    Text(String),
    Number(i64),
    Flag(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryState {
    variables: BTreeMap<String, StateValue>,
}

impl StoryState {
    pub fn new(initial: BTreeMap<String, StateValue>) -> Self {
        Self {
            variables: initial,
        }
    }

    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.variables.get(key)
    }

    pub fn set(&mut self, key: String, value: StateValue) {
        self.variables.insert(key, value);
    }

    pub fn add(&mut self, key: &str, amount: i64) {
        if let Some(StateValue::Number(n)) = self.variables.get(key) {
            let new_val = n + amount;
            self.variables.insert(key.to_string(), StateValue::Number(new_val));
        }
    }

    pub fn variables(&self) -> &BTreeMap<String, StateValue> {
        &self.variables
    }

    pub fn canonical_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for (k, v) in &self.variables {
            let val_str = match v {
                StateValue::Text(s) => format!("\"{}\"", s),
                StateValue::Number(n) => n.to_string(),
                StateValue::Flag(b) => b.to_string(),
            };
            parts.push(format!("{}={}", k, val_str));
        }
        parts.join(",")
    }
}
