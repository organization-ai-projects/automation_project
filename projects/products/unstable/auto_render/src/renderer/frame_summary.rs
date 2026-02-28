use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct FrameSummary {
    pub tick_id: u64,
    pub entity_count: usize,
    pub camera_fov_deg: f64,
    pub light_count: usize,
}
