use crate::zoning::ZoneKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CitizenGroup {
    pub zone: ZoneKind,
    pub count: u64,
    pub happiness: i32,
}
