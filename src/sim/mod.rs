mod boop;
mod world;

use boop::Boop;
use world::World;

pub use world::{Coordinate, Direction};

use crate::build_vec;

pub struct Simulation {
    world: World,
    boops: Vec<Boop>,
    hidden_neurons: u8,
}

impl Simulation {
    pub fn new(size: u8, boops: usize, synapses: u16, hidden_neurons: u8) -> Self {
        Self {
            world: World::new(size, boops, 4),
            boops: build_vec!(|| Boop::new(synapses, hidden_neurons), boops),
            hidden_neurons,
        }
    }

    pub fn step(&mut self) {
        // TODO: The sequence of actions may interfer with each other
        for index in 0..self.boops.len() {
            // TODO: If an action kills, this loop must be careful
            // SAFETY: `boop` does not get moved or dropped
            let boop: *mut Boop = self.boop_mut(Index(index));
            unsafe { (*boop).mind_mut().react(self, Index(index)) };
        }
    }

    pub fn next_generation(&mut self) -> bool {
        use rand::seq::SliceRandom;

        let count = self.boops.len();

        let mut survivors = (0..count)
            .filter(|i| self.world.on_food(Index(*i)))
            .collect::<Vec<_>>();

        if survivors.is_empty() {
            return false;
        }

        let mut rng = rand::thread_rng();
        survivors.shuffle(&mut rng);

        let mut spawn = Vec::with_capacity(count);

        for _ in 0..count {
            let father = *survivors.choose(&mut rng).unwrap();
            let mother = *survivors.choose(&mut rng).unwrap();

            spawn.push(self.boop(Index(father)).mate(
                self.boop(Index(mother)),
                0.001,
                self.hidden_neurons,
            ));
        }

        self.world = World::new(self.size(), count, 4);
        self.boops = spawn;
        true
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.world.size()
    }

    #[inline]
    pub fn fodder(&self) -> impl Iterator<Item = &Coordinate> {
        self.world.fodder()
    }

    pub fn boops(&self) -> impl Iterator<Item = Accessor<'_>> {
        self.boops
            .iter()
            .enumerate()
            .map(|(i, b)| Accessor(b, self.world.boop(Index(i))))
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
