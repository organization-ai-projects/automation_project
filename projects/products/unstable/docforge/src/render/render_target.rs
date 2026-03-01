#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RenderTarget {
    Html,
    Text,
}
