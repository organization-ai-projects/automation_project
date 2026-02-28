use super::{RendererBackend, RendererError};
use crate::world::WorldState;

pub struct NoopRenderer;

impl RendererBackend for NoopRenderer {
    fn render_frame(&self, world: &WorldState) -> Result<(), RendererError> {
        let _entity_count = world.query_entities().len();
        let _camera_fov = world.get_camera().fov_deg;
        let _lights_count = world.get_lighting().lights.len();
        Ok(())
    }
}
