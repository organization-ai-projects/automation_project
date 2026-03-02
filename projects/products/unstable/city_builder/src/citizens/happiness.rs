#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Happiness {
    pub value: i32,
}

impl Happiness {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
