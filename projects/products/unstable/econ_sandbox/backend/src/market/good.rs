use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Good {
    Food,
    Tools,
    Luxuries,
}

impl Good {
    pub fn all() -> &'static [Good] {
        &[Good::Food, Good::Tools, Good::Luxuries]
    }
}

impl std::fmt::Display for Good {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Good::Food => write!(f, "Food"),
            Good::Tools => write!(f, "Tools"),
            Good::Luxuries => write!(f, "Luxuries"),
        }
    }
}
