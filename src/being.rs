use super::geo::Direction;
use super::neural::Brain;

pub struct Being {
    direction: Direction,
    brain: Brain,
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
