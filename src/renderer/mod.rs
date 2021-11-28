use super::sim::Simulation;

// TODO: Put behind feature for 256 color term
mod terminal;
pub use terminal::Terminal;

pub trait Renderer {
    fn render(&self, simulation: &Simulation);
}
