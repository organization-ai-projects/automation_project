use super::RendererError;
use crate::world::WorldState;

pub trait RendererBackend {
    fn render_frame(&self, world: &WorldState) -> Result<(), RendererError>;
}
