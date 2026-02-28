use super::{RendererBackend, RendererError};
use crate::world::WorldState;

pub struct NoopRenderer;

impl RendererBackend for NoopRenderer {
    fn render_frame(&self, _world: &WorldState) -> Result<(), RendererError> {
        Ok(())
    }
}
