use crate::zoning::zone_kind::ZoneKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialZone {
    pub x: u32,
    pub y: u32,
    pub kind: ZoneKind,
}
