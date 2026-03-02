#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "PascalCase")]
pub enum ServiceKind {
    Power,
    Water,
    Health,
    Police,
    Fire,
}
