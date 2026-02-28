#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildingKind {
    House,
    Apartment,
    Shop,
    Factory,
    Office,
}
