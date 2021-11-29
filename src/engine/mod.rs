use super::sim::Simulation;

// TODO: Put behind feature for 256 color term
mod terminal;
pub use terminal::Terminal;

mod quad;
pub use quad::Quad;

pub trait Engine {
    fn start(self, simulation: Simulation);
}
