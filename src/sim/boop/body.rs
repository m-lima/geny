use super::super::Direction;

pub struct Body {
    direction: Direction,
}

impl Body {
    pub fn new() -> Self {
        Self {
            direction: Direction::random(),
        }
    }

    #[inline]
    pub fn direction(&self) -> Direction {
        self.direction
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
