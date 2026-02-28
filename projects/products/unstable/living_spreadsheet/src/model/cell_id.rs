use crate::diagnostics::error::SpreadsheetError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CellId {
    pub row: u32,
    pub col: u32,
}

impl CellId {
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }

    /// Parse A1 notation (e.g. "A1", "B2", "AA10").
    pub fn from_a1(s: &str) -> Result<Self, SpreadsheetError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(SpreadsheetError::ParseError(format!(
                "empty cell reference"
            )));
        }

        let col_end = s.bytes().take_while(|b| b.is_ascii_alphabetic()).count();
        if col_end == 0 {
            return Err(SpreadsheetError::ParseError(format!(
                "invalid cell reference: {}",
                s
            )));
        }

        let col_str = &s[..col_end];
        let row_str = &s[col_end..];

        if row_str.is_empty() {
            return Err(SpreadsheetError::ParseError(format!(
                "missing row number in: {}",
                s
            )));
        }

        let row: u32 = row_str
            .parse()
            .map_err(|_| SpreadsheetError::ParseError(format!("invalid row number in: {}", s)))?;

        if row == 0 {
            return Err(SpreadsheetError::ParseError(format!(
                "row must be >= 1 in: {}",
                s
            )));
        }

        let mut col: u32 = 0;
        for ch in col_str.chars() {
            let ch = ch.to_ascii_uppercase();
            if !ch.is_ascii_uppercase() {
                return Err(SpreadsheetError::ParseError(format!(
                    "invalid column char in: {}",
                    s
                )));
            }
            col = col * 26 + (ch as u32 - 'A' as u32 + 1);
        }

        Ok(CellId {
            row: row - 1,
            col: col - 1,
        })
    }

    /// Convert to A1 notation.
    pub fn to_a1(&self) -> String {
        let mut col = self.col + 1;
        let mut col_str = String::new();
        while col > 0 {
            let rem = (col - 1) % 26;
            col_str.insert(0, (b'A' + rem as u8) as char);
            col = (col - 1) / 26;
        }
        format!("{}{}", col_str, self.row + 1)
    }
}

impl fmt::Display for CellId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_a1())
    }
}
