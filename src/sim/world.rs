use super::Index;

pub struct World {
    size: u8,
    coord: Vec<Coordinate>,
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

        let coord = (0..size)
            .flat_map(|x| (0..size).map(|y| Coordinate::new(x, y)).collect::<Vec<_>>())
            .choose_multiple(&mut rand::thread_rng(), count as usize);

        Self { size, coord }
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.size
    }

    #[inline]
    pub fn coord(&self, index: Index) -> Coordinate {
        unsafe { *self.coord.get_unchecked(index.0) }
    }

    pub fn advance(&mut self, index: Index, direction: Direction) {
        // SAFETY: Called from a sim step. Always within bounds
        let coord = unsafe { self.coord.get_unchecked(index.0) };

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

        if !self.coord.iter().enumerate().any(|(_, c)| *c == dest) {
            // SAFETY: Previously checked for bounds
            unsafe { *self.coord.get_unchecked_mut(index.0) = dest };
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.coord.swap_remove(index);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl Direction {
    pub fn from(value: u8) -> Self {
        match value & 0b11 {
            3 => Self::West,
            2 => Self::South,
            1 => Self::East,
            _ => Self::North,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Coordinate(u16);

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Self(u16::from(x) | (u16::from(y) << 8))
    }

    // ALLOWED: There is a bit mask already limiting the result
    #[allow(clippy::cast_possible_truncation)]
    pub fn x(self) -> u8 {
        (self.0 & 0xff) as u8
    }

    // ALLOWED: There is a bit mask already limiting the result
    #[allow(clippy::cast_possible_truncation)]
    pub fn y(self) -> u8 {
        (self.0 >> 8 & 0xff) as u8
    }

    pub fn neighbor(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self(self.0 - 0x100),
            Direction::East => Self(self.0 + 1),
            Direction::South => Self(self.0 + 0x100),
            Direction::West => Self(self.0 - 1),
        }
    }
}
