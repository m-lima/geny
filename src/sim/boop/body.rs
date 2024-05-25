use super::super::Direction;

pub struct Body {
    direction: Direction,
    energy: f32,
}

impl Body {
    pub fn new() -> Self {
        Self {
            direction: Direction::random(),
            energy: f32::from(u8::MAX),
        }
    }

    #[inline]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    #[inline]
    pub fn drain(&mut self, amount: f32) {
        self.energy -= amount;
    }

    #[inline]
    pub fn turn_right(&mut self, amount: f32) {
        self.direction += amount;
    }

    #[inline]
    pub fn turn_left(&mut self, amount: f32) {
        self.direction -= amount;
    }

    #[inline]
    pub fn energy(&self) -> f32 {
        self.energy
    }

    #[inline]
    pub fn restore_energy(&mut self, energy: u8) {
        self.energy += f32::from(energy);
        self.energy = self.energy.min(f32::from(u8::MAX) * 16.0);
    }
}
