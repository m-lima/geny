use super::sim::Simulation;

// TODO: Put behind feature for 256 color term
mod terminal;
pub use terminal::Terminal;

pub trait Renderer {
    fn render<const H: u8, const S: usize>(&self, simulation: &Simulation<H, S>);
}
