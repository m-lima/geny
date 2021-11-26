use super::being::Being;
use super::geo::{Coordinate, Direction};

pub struct World {
    size: u8,
    beings: Vec<Being>,
    coordinates: Vec<Coordinate>,
    // TODO: Consider maybe using a grid here as a cache to represent the locations
}

impl World {
    #[inline]
    pub fn new(size: u8, count: u16) -> Self {
        use rand::seq::IteratorRandom;

        let count = count.min(u16::from(size) * u16::from(size));

        let beings = (0..count).map(|_| Being::new()).collect();
        let coordinates = (0..size)
            .flat_map(|x| (0..size).map(|y| Coordinate::new(x, y)).collect::<Vec<_>>())
            .choose_multiple(&mut rand::thread_rng(), count as usize);

        Self {
            size,
            beings,
            coordinates,
        }
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.size
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.beings.len()
    }

    pub fn being(&self, index: usize) -> &Being {
        &self.beings[index]
    }

    pub fn being_mut(&mut self, index: usize) -> &mut Being {
        &mut self.beings[index]
    }

    pub fn coordinate(&self, index: usize) -> Coordinate {
        self.coordinates[index]
    }

    pub fn coordinates(&self) -> impl Iterator<Item = &Coordinate> {
        self.coordinates.iter()
    }

    pub fn advance(&mut self, being: usize, direction: Direction) {
        let coord = self.coordinates[being];

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

        if !self.coordinates.iter().enumerate().any(|(_, c)| *c == dest) {
            self.coordinates[being] = dest;
        }
    }
}
