#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RngDraw {
    pub context: String,
    pub value: u64,
}
