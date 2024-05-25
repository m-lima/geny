use super::Index;

pub struct World {
    size: u8,
    sizef: f32,
    boops: Vec<Coordinate>,
    food: Vec<(Coordinate, u8)>,
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
            boops: build_vec!(|| Coordinate::random(sizef), count),
            food: build_vec!(|| (Coordinate::random(sizef), u8::MAX), food_count),
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
        self.food.iter().map(|(c, _)| c)
    }

    pub fn eat(&mut self, index: Index) -> Option<u8> {
        let boop = self.boop(index);
        let boop_x = boop.x();
        for (food, energy) in &mut self.food {
            if (food.x() - boop_x).abs() < 1.0 && food.distance(boop) < 1. {
                if *energy <= 32 {
                    *energy = 0;
                    *food = Coordinate::random(self.sizef);
                } else {
                    *energy -= 32;
                }
                return Some(32);
            }
        }
        None
    }

    pub fn regenerate_food(&mut self) {
        for (food, energy) in &mut self.food {
            if *energy == 0 {
                *energy = u8::MAX;
                *food = Coordinate::random(self.sizef);
            }
        }
    }

    pub fn on_food(&self, index: Index) -> bool {
        let boop = self.boop(index);
        let boop_x = boop.x();
        for (food, _) in &self.food {
            if (food.x() - boop_x).abs() < 1.0 && food.distance(boop) < 1. {
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self, index: Index, speed: f32, direction: Direction) {
        let mut coord = self.boop(index);

        coord.translate(direction, speed, self.sizef);

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

    fn random(size: f32) -> Self {
        Self::new(rand::random::<f32>() * size, rand::random::<f32>() * size)
    }

    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    // ALLOWED: Coord is never negative or out of bounds, due to `translate`
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn x_index(self) -> u8 {
        self.0 as u8
    }

    // ALLOWED: Coord is never negative or out of bounds, due to `translate`
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn y_index(self) -> u8 {
        self.1 as u8
    }

    pub fn dir_from(self, rhs: Self) -> Direction {
        let vec = self - rhs;
        Direction(vec.1.atan2(vec.0))
    }

    fn translate(&mut self, dir: Direction, amount: f32, max: f32) {
        self.0 += dir.0.cos() * amount;
        self.1 += dir.0.sin() * amount;

        self.0 = self.0.clamp(0.0, max);
        self.1 = self.1.clamp(0.0, max);
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
        assert!(Coordinate(1., 0.).dir_from(c1) == Direction(0.));
        assert!(Coordinate(0., 1.).dir_from(c1) == Direction(std::f32::consts::FRAC_PI_2));
        assert!(
            Coordinate(0., 1.).dir_from(c1) - Direction(std::f32::consts::FRAC_PI_2)
                == Direction(0.)
        );
    }
}
