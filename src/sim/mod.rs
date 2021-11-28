mod gene;
mod world;

use super::being::Being;
pub use gene::{Genome, Input, Output};
pub use world::World;

pub struct Simulation<const H: u8, const S: usize> {
    world: World,
    beings: Vec<Being<H, S>>,
}

impl<const H: u8, const S: usize> Simulation<H, S> {
    #[inline]
    pub fn world(&self) -> &World {
        &self.world
    }

    #[inline]
    pub fn indices(&self) -> impl Iterator<Item = Index> {
        (0..self.beings.len()).map(Index)
    }

    #[inline]
    pub fn being(&self, index: Index) -> &Being<H, S> {
        unsafe { self.beings.get_unchecked(index.0) }
    }

    #[inline]
    fn being_mut(&mut self, index: Index) -> &mut Being<H, S> {
        unsafe { self.beings.get_unchecked_mut(index.0) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Index(usize);
