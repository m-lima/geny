mod renderer {
    use super::world::World;

    pub trait Renderer {
        fn render(&self, world: &World);
    }

    // TODO: Put behind feature for 256 color term
    mod terminal {
        use super::super::being::Being;
        use super::{Renderer, World};
        pub struct Terminal;

        impl Renderer for Terminal {
            fn render(&self, world: &World) {
                let mut buffer = vec![
                    vec![Option::<&Being>::None; world.size() as usize];
                    world.size() as usize
                ];

                for (being, coord) in world
                    .coordinates()
                    .enumerate()
                    .map(|(i, c)| (world.being(i), c))
                {
                    buffer[coord.y() as usize][coord.x() as usize] = Some(being);
                }

                for row in buffer {
                    for cell in row {
                        if let Some(being) = cell {
                            let mut color = being.as_u24();
                            let b = color & 0xff;
                            color >>= 8;
                            let g = color & 0xff;
                            color >>= 8;
                            let r = color & 0xff;
                            print!("[38;2;{:03};{:03};{:03}m\u{2588}\u{2588}[m", r, g, b);
                        } else {
                            print!("  ");
                        }
                    }
                    println!();
                }
                println!();
            }
        }
    }

    pub use terminal::Terminal;
}

mod world {
    use super::being::Being;
    use super::grid::{Coordinate, Direction};

    pub struct World {
        size: u8,
        beings: Vec<Being>,
        coordinates: Vec<Coordinate>,
    }

    impl World {
        #[inline]
        pub fn new(size: u8, count: u16) -> Self {
            use rand::seq::IteratorRandom;

            // TODO: This assert is kinda ugly
            assert!(count < u16::from(size) * u16::from(size));

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
}

mod grid {
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

    impl From<(u8, u8)> for Coordinate {
        fn from((x, y): (u8, u8)) -> Self {
            Self::new(x, y)
        }
    }
}

mod brain {
    use super::being::Being;
    use super::grid::Direction;
    use super::world::World;

    enum Input {
        Random,
        DirectionVertical,
        DirectionHorizontal,
    }

    impl Input {
        fn spike(&self, world: &World, index: usize) -> f32 {
            match self {
                Self::Random => rand::random(),
                Self::DirectionVertical => match world.being(index).direction() {
                    Direction::East | Direction::West => 0.0,
                    Direction::North => 1.0,
                    Direction::South => -1.0,
                },
                Self::DirectionHorizontal => match world.being(index).direction() {
                    Direction::North | Direction::South => 0.0,
                    Direction::East => 1.0,
                    Direction::West => -1.0,
                },
            }
        }
    }

    enum Output {
        Noop,
        TurnLeft,
        TurnRight,
        Advance,
    }

    impl Output {
        fn spike(&self, world: &mut World, index: usize) {
            match self {
                Self::Noop => {}
                Self::TurnLeft => world.being_mut(index).turn_left(),
                Self::TurnRight => world.being_mut(index).turn_right(),
                Self::Advance => {
                    let direction = world.being(index).direction();
                    world.advance(index, direction);
                }
            }
        }
    }

    enum Axon {
        Input(Input),
    }

    enum Dentrite {
        Output(Output),
    }

    struct Synapsis<'a> {
        axon: &'a Axon,
        dentrite: &'a Dentrite,
    }

    struct Brain {}
}

mod being {
    use super::grid::Direction;

    pub struct Being {
        direction: Direction,
    }

    impl Being {
        pub fn new() -> Self {
            Self {
                direction: Direction::from(rand::random()),
            }
        }

        pub fn turn_right(&mut self) {
            self.direction = Direction::from(self.direction as u8 + 1);
        }

        pub fn turn_left(&mut self) {
            self.direction = Direction::from((self.direction as u8).wrapping_sub(1));
        }

        pub fn as_u24(&self) -> u32 {
            rand::random()
        }

        #[inline]
        pub fn direction(&self) -> Direction {
            self.direction
        }

        pub fn step(&self, _all_info_needed: usize) {}
    }

    #[cfg(test)]
    mod test {
        use super::{Being, Direction};

        #[test]
        fn turn() {
            fn turn_right(being: &mut Being) {
                let direction = being.direction();
                being.turn_right();
                match direction {
                    Direction::North => assert_eq!(being.direction(), Direction::East),
                    Direction::East => assert_eq!(being.direction(), Direction::South),
                    Direction::South => assert_eq!(being.direction(), Direction::West),
                    Direction::West => assert_eq!(being.direction(), Direction::North),
                }
            }

            fn turn_left(being: &mut Being) {
                let direction = being.direction();
                being.turn_left();
                match direction {
                    Direction::North => assert_eq!(being.direction(), Direction::West),
                    Direction::West => assert_eq!(being.direction(), Direction::South),
                    Direction::South => assert_eq!(being.direction(), Direction::East),
                    Direction::East => assert_eq!(being.direction(), Direction::North),
                }
            }

            let mut being = Being {
                direction: Direction::from(0),
            };

            for i in 0..8 {
                println!("{}", i);
                turn_right(&mut being);
            }

            for i in 0..16 {
                println!("{}", i);
                turn_left(&mut being);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let size = args
        .next()
        .ok_or(anyhow::anyhow!("No size provided"))?
        .parse()?;

    let count = args
        .next()
        .ok_or(anyhow::anyhow!("No count provided"))?
        .parse()?;

    let grid = world::World::new(size, count);

    renderer::Renderer::render(&renderer::Terminal, &grid);

    Ok(())
}
