use super::{RendererBackend, RendererError};
use crate::world::WorldState;
use serde::Serialize;
use std::path::PathBuf;

pub struct FrameDumpRenderer {
    output_dir: PathBuf,
}

impl FrameDumpRenderer {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
}

#[derive(Serialize)]
struct FrameSummary {
    tick_id: u64,
    entity_count: usize,
    camera_fov_deg: f64,
    light_count: usize,
}

impl RendererBackend for FrameDumpRenderer {
    fn render_frame(&self, world: &WorldState) -> Result<(), RendererError> {
        if self.output_dir.as_os_str().is_empty() {
            return Err(RendererError::NotInitialized);
        }

        std::fs::create_dir_all(&self.output_dir).map_err(|_| RendererError::Unsupported)?;

        let summary = FrameSummary {
            tick_id: world.tick_id,
            entity_count: world.entities.len(),
            camera_fov_deg: world.camera.fov_deg,
            light_count: world.lighting.lights.len(),
        };

        let output_path = self
            .output_dir
            .join(format!("frame-{}.json", summary.tick_id));
        let payload =
            serde_json::to_string_pretty(&summary).map_err(|_| RendererError::Unsupported)?;
        std::fs::write(output_path, payload).map_err(|_| RendererError::Unsupported)
    }
}
