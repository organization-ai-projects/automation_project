use super::{RendererBackend, RendererError};
use crate::renderer::frame_summary::FrameSummary;
use crate::world::WorldState;
use std::path::PathBuf;

pub struct FrameDumpRenderer {
    output_dir: PathBuf,
}

impl FrameDumpRenderer {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
}

impl RendererBackend for FrameDumpRenderer {
    fn render_frame(&self, world: &WorldState) -> Result<(), RendererError> {
        if self.output_dir.as_os_str().is_empty() {
            return Err(RendererError::NotInitialized);
        }

        std::fs::create_dir_all(&self.output_dir).map_err(|_| RendererError::Unsupported)?;

        let summary = FrameSummary {
            tick_id: world.tick_id,
            entity_count: world.query_entities().len(),
            camera_fov_deg: world.get_camera().fov_deg,
            light_count: world.get_lighting().lights.len(),
        };

        let output_path = self
            .output_dir
            .join(format!("frame-{}.json", summary.tick_id));
        let payload =
            serde_json::to_string_pretty(&summary).map_err(|_| RendererError::Unsupported)?;
        std::fs::write(output_path, payload).map_err(|_| RendererError::Unsupported)
    }
}
