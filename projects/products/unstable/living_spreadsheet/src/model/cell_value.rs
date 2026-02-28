use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Error(String),
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Empty => write!(f, ""),
            CellValue::Number(n) => write!(f, "{}", n),
            CellValue::Text(s) => write!(f, "{}", s),
            CellValue::Error(e) => write!(f, "#ERR:{}", e),
        }
    }
}
