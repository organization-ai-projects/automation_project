use serde::{Deserialize, Serialize};

/// A named item type that can be produced, transported, and consumed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item({})", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_name() {
        let item = Item::new("iron_ore");
        assert_eq!(item.name, "iron_ore");
    }
}
