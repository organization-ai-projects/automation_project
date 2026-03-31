use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpertType {
    Antivirus,
    Firewall,
    SymbolicAnalyzer,
    Hybrid,
}
