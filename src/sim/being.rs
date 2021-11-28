use super::super::geo::Direction;
use super::super::neural::Brain;
use super::{Input, Output};

pub struct Being {
    direction: Direction,
    // huger
    // resilience
    // strength
    brain: Brain<Input, Output, { super::HIDDEN_NEURONS }>,
}

impl Being {
    pub(super) fn new(brain: Brain<Input, Output, { super::HIDDEN_NEURONS }>) -> Self {
        Self {
            direction: Direction::from(rand::random()),
            brain,
        }
    }

    pub(super) fn brain(&mut self) -> &mut Brain<Input, Output, { super::HIDDEN_NEURONS }> {
        &mut self.brain
    }

    pub(super) fn turn_right(&mut self) {
        self.direction = Direction::from(self.direction as u8 + 1);
    }

    pub(super) fn turn_left(&mut self) {
        self.direction = Direction::from((self.direction as u8).wrapping_sub(1));
    }

    #[inline]
    pub fn direction(&self) -> Direction {
        self.direction
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
            brain: Brain::new([].into_iter()),
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
