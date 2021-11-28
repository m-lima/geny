use super::geo::Direction;
use super::neural::Brain;
use super::sim::{Genome, Input, Output};

// TODO: Remove this type param
pub struct Being<const H: u8, const S: usize> {
    direction: Direction,
    // huger
    // resilience
    // strength
    brain: Brain<Input, Output>,
    genome: Genome<H, S>,
}

impl<const H: u8, const S: usize> Being<H, S> {
    pub fn new() -> Self {
        // TODO: This
        unimplemented!()
        // let brain = Brain::new(genome.iter().map(Gene::to_axon));
        // Self {
        //     direction: Direction::from(rand::random()),
        //     brain,
        //     genome,
        // }
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
    use super::{Being, Brain, Direction};

    #[test]
    fn turn() {
        fn turn_right(being: &mut Being<0, 0>) {
            let direction = being.direction();
            being.turn_right();
            match direction {
                Direction::North => assert_eq!(being.direction(), Direction::East),
                Direction::East => assert_eq!(being.direction(), Direction::South),
                Direction::South => assert_eq!(being.direction(), Direction::West),
                Direction::West => assert_eq!(being.direction(), Direction::North),
            }
        }

        fn turn_left(being: &mut Being<0, 0>) {
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
