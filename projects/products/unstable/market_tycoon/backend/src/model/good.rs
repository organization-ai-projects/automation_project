use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Good {
    Widget,
    Gadget,
    Gizmo,
}

impl std::fmt::Display for Good {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Good::Widget => write!(f, "Widget"),
            Good::Gadget => write!(f, "Gadget"),
            Good::Gizmo => write!(f, "Gizmo"),
        }
    }
}
