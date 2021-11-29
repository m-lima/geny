use super::super::Direction;

pub struct Body {
    direction: Direction,
    age: u8,
    hunger: u8,
}

impl Body {
    pub fn new() -> Self {
        Self {
            direction: Direction::from(rand::random()),
            age: 0,
            hunger: 0,
        }
    }

    #[inline]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    #[inline]
    pub fn age(&self) -> u8 {
        self.age
    }

    #[inline]
    pub fn hunger(&self) -> u8 {
        self.hunger
    }

    #[inline]
    pub fn tick(&mut self) {
        self.age += 1;
        self.hunger += 1;
    }

    #[inline]
    pub fn turn_right(&mut self) {
        self.direction = Direction::from(self.direction as u8 + 1);
    }

    #[inline]
    pub fn turn_left(&mut self) {
        self.direction = Direction::from((self.direction as u8).wrapping_sub(1));
    }
}

#[cfg(test)]
mod test {
    use super::{Body, Direction};

    #[test]
    fn turn() {
        fn turn_right(body: &mut Body) {
            let direction = body.direction();
            body.turn_right();
            match direction {
                Direction::North => assert_eq!(body.direction(), Direction::East),
                Direction::East => assert_eq!(body.direction(), Direction::South),
                Direction::South => assert_eq!(body.direction(), Direction::West),
                Direction::West => assert_eq!(body.direction(), Direction::North),
            }
        }

        fn turn_left(body: &mut Body) {
            let direction = body.direction();
            body.turn_left();
            match direction {
                Direction::North => assert_eq!(body.direction(), Direction::West),
                Direction::West => assert_eq!(body.direction(), Direction::South),
                Direction::South => assert_eq!(body.direction(), Direction::East),
                Direction::East => assert_eq!(body.direction(), Direction::North),
            }
        }

        let mut body = Body::new();
        body.direction = Direction::from(0);

        for _ in 0..8 {
            turn_right(&mut body);
        }

        for _ in 0..16 {
            turn_left(&mut body);
        }
    }
}
