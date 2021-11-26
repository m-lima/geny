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
