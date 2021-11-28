// TODO: Move geo in here
use super::super::geo::{Coordinate, Direction};
use super::Index;

pub struct World {
    size: u8,
    beings: Vec<Coordinate>,
    // Walls
    // Foods
    // Lava
    // TODO: Consider maybe using a grid here as a cache to represent the locations
}

impl World {
    #[inline]
    pub fn new(size: u8, count: u16) -> Self {
        use rand::seq::IteratorRandom;

        let count = count.min(u16::from(size) * u16::from(size));

        let beings = (0..size)
            .flat_map(|x| (0..size).map(|y| Coordinate::new(x, y)).collect::<Vec<_>>())
            .choose_multiple(&mut rand::thread_rng(), count as usize);

        Self { size, beings }
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn being(&self, index: Index) -> Coordinate {
        unsafe { *self.beings.get_unchecked(index.0) }
    }

    pub fn advance(&mut self, index: Index, direction: Direction) {
        // SAFETY: Called from a sim step. Always within bounds
        let coord = unsafe { self.beings.get_unchecked(index.0) };

        let dest = match direction {
            Direction::North if coord.y() > 0 => coord.neighbor(Direction::North),
            Direction::North => return,
            Direction::East if coord.x() < self.size - 1 => coord.neighbor(Direction::East),
            Direction::East => return,
            Direction::South if coord.y() < self.size - 1 => coord.neighbor(Direction::South),
            Direction::South => return,
            Direction::West if coord.x() > 0 => coord.neighbor(Direction::West),
            Direction::West => return,
        };

        if !self.beings.iter().enumerate().any(|(_, c)| *c == dest) {
            // SAFETY: Previously checked for bounds
            unsafe { *self.beings.get_unchecked_mut(index.0) = dest };
        }
    }
}
