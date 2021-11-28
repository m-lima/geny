use super::geo::Direction;
use super::neural::{Brain, Gene, Stimuli};
use super::world::World;

pub struct Being {
    direction: Direction,
    brain: Brain<Input, Output>,
    genome: Vec<Gene<Input, Output, 2>>,
}

impl Being {
    pub fn new(genome: Vec<Gene<Input, Output, 2>>) -> Self {
        Self {
            direction: Direction::from(rand::random()),
            brain: Brain::new(&genome),
            genome,
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
        // use std::hash::{Hash, Hasher};

        // let mut state = std::collections::hash_map::DefaultHasher::default();
        // for gene in self.genome {
        //     gene.hash(&mut state);
        // }

        // state.finish() as u32
    }

    #[inline]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn step(&self, _all_info_needed: usize) {}
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Input {
    Random = 0,
    DirectionVertical,
    DirectionHorizontal,
}

impl Input {
    fn from(index: usize) -> Self {
        // ALLOWED: This should come already clamped from the neural net
        // SAFTEY: This should come already clamped from the neural net
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index as u8)
        }
    }

    fn sense(self, world: &World, index: usize) -> Stimuli {
        match self {
            Self::Random => Stimuli::cap(rand::random::<f32>()),
            Self::DirectionVertical => match world.being(index).direction() {
                Direction::East | Direction::West => Stimuli::half(),
                Direction::North => true.into(),
                Direction::South => false.into(),
            },
            Self::DirectionHorizontal => match world.being(index).direction() {
                Direction::North | Direction::South => Stimuli::half(),
                Direction::East => true.into(),
                Direction::West => false.into(),
            },
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Output {
    Noop = 0,
    TurnLeft,
    TurnRight,
    Advance,
}

impl Output {
    fn from(index: usize) -> Self {
        // ALLOWED: This should come already clamped from the neural net
        // SAFTEY: This should come already clamped from the neural net
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index as u8)
        }
    }

    fn act(self, world: &mut World, index: usize) {
        match self {
            Self::Noop => println!("Noop"),
            Self::TurnLeft => {
                println!("Tunr left");
                world.being_mut(index).turn_left()
            }
            Self::TurnRight => {
                println!("Tunr right");
                world.being_mut(index).turn_right()
            }
            Self::Advance => {
                println!("Advancing");
                let direction = world.being(index).direction();
                world.advance(index, direction);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Being, Brain, Direction};

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
            brain: Brain::new(&[]),
            genome: vec![],
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
