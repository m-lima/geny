use super::world::World;

// TODO: Put behind feature for 256 color term
mod terminal;
pub use terminal::Terminal;

pub trait Renderer {
    fn render(&self, world: &World);
}
