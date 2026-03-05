use crate::services::service_kind::ServiceKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InitialService {
    pub x: u32,
    pub y: u32,
    pub kind: ServiceKind,
    pub coverage_radius: u32,
}
