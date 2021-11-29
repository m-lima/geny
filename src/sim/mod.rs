mod boop;
mod world;

use boop::Boop;
use world::World;

pub use world::{Coordinate, Direction};

use crate::build_vec;

pub struct Simulation {
    world: World,
    boops: Vec<Boop>,
}

impl Simulation {
    pub fn new(size: u8, boops: u16, synapses: u16, hidden_neurons: u8) -> Self {
        Self {
            world: World::new(size, boops),
            boops: build_vec!(|| Boop::new(synapses, hidden_neurons), boops),
        }
    }

    pub fn step(&mut self) -> bool {
        // TODO: The sequence of actions may interfer with each other
        for index in 0..self.boops.len() {
            let boop = self.boop_mut(Index(index));
            boop.tick();

            // TODO: If an action kills, this loop must be careful
            // SAFETY: `boop` does not get moved or dropped
            let boop: *mut Boop = self.boop_mut(Index(index));
            unsafe { (*boop).mind_mut().react(self, Index(index)) };
        }

        self.reap()
    }

    fn reap(&mut self) -> bool {
        let mut i = 0;
        loop {
            if i == self.boops.len() {
                break;
            }

            let dead = {
                let boop = self.boop(Index(i));
                boop.age() == 255 || boop.hunger() == 255
            };

            if dead {
                self.boops.swap_remove(i);
                self.world.remove(i);
            } else {
                i += 1;
            }
        }

        !self.boops.is_empty()
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.world.size()
    }

    pub fn boops(&self) -> impl Iterator<Item = Accessor<'_>> {
        self.boops
            .iter()
            .enumerate()
            .map(|(i, b)| Accessor(b, self.world.coord(Index(i))))
    }

    #[inline]
    fn boop(&self, index: Index) -> &Boop {
        unsafe { self.boops.get_unchecked(index.0) }
    }

    #[inline]
    fn boop_mut(&mut self, index: Index) -> &mut Boop {
        unsafe { self.boops.get_unchecked_mut(index.0) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Index(usize);

pub struct Accessor<'a>(&'a Boop, Coordinate);

impl Accessor<'_> {
    pub fn coordinate(&self) -> Coordinate {
        self.1
    }
}

impl std::ops::Deref for Accessor<'_> {
    type Target = Boop;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
