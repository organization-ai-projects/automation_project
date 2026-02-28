#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Budget {
    pub balance: i64,
    pub income: i64,
    pub expenses: i64,
}

impl Budget {
    pub fn new() -> Self {
        Self { balance: 10000, income: 0, expenses: 0 }
    }
}

impl Default for Budget {
    fn default() -> Self {
        Self::new()
    }
}
