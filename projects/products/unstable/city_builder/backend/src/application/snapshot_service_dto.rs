use crate::services;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotServiceDto {
    pub x: u32,
    pub y: u32,
    pub kind: services::service_kind::ServiceKind,
    pub coverage_radius: u32,
}
