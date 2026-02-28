#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ZoneKind {
    Residential,
    Commercial,
    Industrial,
    None,
}
