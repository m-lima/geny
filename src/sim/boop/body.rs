use super::super::Direction;

pub struct Body {
    direction: Direction,
    age: u8,
    hunger: u8,
}

impl Body {
    pub fn new() -> Self {
        Self {
            direction: Direction::new(rand::random()),
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
    pub fn turn_right(&mut self, amount: f32) {
        self.direction += amount;
    }

    #[inline]
    pub fn turn_left(&mut self, amount: f32) {
        self.direction -= amount;
    }
}
