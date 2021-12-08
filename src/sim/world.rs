use super::Index;

use crate::build_vec;

pub struct World {
    size: u8,
    sizef: f32,
    boops: Vec<Coordinate>,
    food: Vec<Coordinate>,
    // Walls
    // Foods
    // Lava
    // TODO: Consider maybe using a grid here as a cache to represent the locations
}

impl World {
    pub fn new(size: u8, count: usize, food_count: usize) -> Self {
        let sizef = f32::from(size - 1);
        Self {
            size,
            sizef,
            boops: build_vec!(
                || Coordinate::new(rand::random::<f32>() * sizef, rand::random::<f32>() * sizef,),
                count
            ),
            food: build_vec!(
                || Coordinate::new(rand::random::<f32>() * sizef, rand::random::<f32>() * sizef,),
                food_count
            ),
        }
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.size
    }

    #[inline]
    pub fn boop(&self, index: Index) -> Coordinate {
        unsafe { *self.boops.get_unchecked(index.0) }
    }

    #[inline]
    fn boop_mut(&mut self, index: Index) -> &mut Coordinate {
        unsafe { self.boops.get_unchecked_mut(index.0) }
    }

    #[inline]
    pub fn fodder(&self) -> impl Iterator<Item = &Coordinate> {
        self.food.iter()
    }

    pub fn on_food(&mut self, index: Index) -> bool {
        for food in &self.food {
            if food.distance(self.boop(index)) < 1. {
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self, index: Index, speed: f32, direction: Direction) {
        let mut coord = self.boop(index);

        coord.translate(direction, speed);

        if coord.0 < 0. {
            coord.0 = 0.;
        } else if coord.0 > self.sizef {
            coord.0 = self.sizef;
        }

        if coord.1 < 0. {
            coord.1 = 0.;
        } else if coord.1 > self.sizef {
            coord.1 = self.sizef;
        }

        *self.boop_mut(index) = coord;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Direction(f32);

impl Direction {
    pub fn random() -> Self {
        Self(rand::random::<f32>() * std::f32::consts::TAU)
    }

    fn desaturate(&mut self) {
        if self.0 > std::f32::consts::TAU {
            self.0 -= std::f32::consts::TAU;
            while self.0 > std::f32::consts::TAU {
                self.0 -= std::f32::consts::TAU;
            }
        } else {
            while self.0 < 0. {
                self.0 += std::f32::consts::TAU;
            }
        }
    }

    pub fn as_rad(self) -> f32 {
        self.0
    }
}

impl std::ops::AddAssign<f32> for Direction {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
        self.desaturate();
    }
}

impl std::ops::SubAssign<f32> for Direction {
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= rhs;
        self.desaturate();
    }
}

impl std::ops::Sub for Direction {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0 -= rhs.0;
        self.desaturate();
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Coordinate(f32, f32);

impl Coordinate {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn dir_from(self, rhs: Self) -> Direction {
        let vec = self - rhs;
        Direction(vec.1.atan2(vec.0))
    }

    pub fn translate(&mut self, dir: Direction, amount: f32) {
        self.0 += dir.0.cos() * amount;
        self.1 += dir.0.sin() * amount;
    }

    pub fn distance(self, rhs: Self) -> f32 {
        let manhattan = self - rhs;
        (manhattan.0 * manhattan.0 + manhattan.1 * manhattan.1).sqrt()
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod test {
    use super::{Coordinate, Direction};

    #[test]
    fn manhattan() {
        for _ in 0..10 {
            let ax = rand::random();
            let ay = rand::random();
            let bx = rand::random();
            let by = rand::random();
            let manhattan = Coordinate(ax, ay) - Coordinate(bx, by);
            assert!(manhattan.0 - (ax - bx) <= f32::EPSILON);
            assert!(manhattan.1 - (ay - by) <= f32::EPSILON);
        }
    }

    #[test]
    fn dir_from() {
        let c1 = Coordinate(0., 0.);
        assert!(Coordinate(1., 0.).dir_from(c1) == Direction(0.),);
        assert!(Coordinate(0., 1.).dir_from(c1) == Direction(0.),);
    }
}
