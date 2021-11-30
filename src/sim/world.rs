use super::Index;

pub struct World {
    size: u8,
    sizef: f32,
    coord: Vec<Coordinate>,
    // Walls
    // Foods
    // Lava
    // TODO: Consider maybe using a grid here as a cache to represent the locations
}

impl World {
    #[inline]
    pub fn new(size: u8, count: u16) -> Self {
        use rand::seq::IteratorRandom;

        let count = count.min(u16::from(size) * u16::from(size));

        let coord = (0..size)
            .flat_map(|x| {
                (0..size)
                    .map(|y| Coordinate::new(f32::from(x), f32::from(y)))
                    .collect::<Vec<_>>()
            })
            .choose_multiple(&mut rand::thread_rng(), count as usize);

        Self {
            size,
            sizef: f32::from(size),
            coord,
        }
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.size
    }

    #[inline]
    pub fn coord(&self, index: Index) -> Coordinate {
        unsafe { *self.coord.get_unchecked(index.0) }
    }

    #[inline]
    fn coord_mut(&mut self, index: Index) -> &mut Coordinate {
        unsafe { self.coord.get_unchecked_mut(index.0) }
    }

    pub fn advance(&mut self, index: Index, speed: f32, direction: Direction) {
        let mut coord = self.coord(index);

        if let Some((neighbor, mut dist)) = self
            .coord
            .iter()
            .filter_map(|c| {
                // ALLOWED: So that we can comment the steps
                #[allow(clippy::if_same_then_else)]
                if *c == coord {
                    // Skip self
                    None
                } else if c.direction(coord).0.abs() - direction.0 > std::f32::consts::PI / 4. {
                    // Skip if not within 45 degrees from `direction`
                    None
                } else {
                    // Only if close enough to collide
                    let dist = c.distance(coord);
                    if dist < speed {
                        Some((c, dist))
                    } else {
                        None
                    }
                }
            })
            .fold(None, |acc, (cc, cd)| {
                if let Some((ac, ad)) = acc {
                    if ad < cd {
                        return Some((ac, ad));
                    }
                }
                Some((cc, cd))
            })
        {
            while dist > 0.01 {
                println!("Advance {}", dist);
                coord.translate(direction, dist);
                dist = neighbor.distance(coord);
            }
        } else {
            coord.translate(direction, speed);
        }

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

        *self.coord_mut(index) = coord;
    }

    pub fn remove(&mut self, index: usize) {
        self.coord.swap_remove(index);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Direction(f32);

impl Direction {
    pub fn new(rads: f32) -> Self {
        Self(rads)
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

    pub fn max(self) -> f32 {
        self.0.max(self.1)
    }

    pub fn min(self) -> f32 {
        self.0.min(self.1)
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    pub fn direction(self, rhs: Self) -> Direction {
        let vec = self - rhs;
        Direction(vec.0.atan2(vec.1))
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

// TODO: Remove if uneeded
// impl std::ops::Add for Coordinate {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0, self.1 + rhs.1)
//     }
// }

// impl std::ops::AddAssign for Coordinate {
//     fn add_assign(&mut self, rhs: Self) {
//         self.0 += rhs.0;
//         self.1 += rhs.1;
//     }
// }

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
