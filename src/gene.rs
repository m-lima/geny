#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Input {
    Random = 0,
    DirectionVertical,
    DirectionHorizontal,
}

impl Input {
    fn from(index: usize) -> Self {
        // ALLOWED: This should come already clamped from the neural net
        // SAFTEY: This should come already clamped from the neural net
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index as u8)
        }
    }

    fn sense(self, world: &world::World, index: usize) -> neural::Signal {
        match self {
            Self::Random => neural::Signal::cap(rand::random::<f32>()),
            Self::DirectionVertical => match world.being(index).direction() {
                geo::Direction::East | geo::Direction::West => neural::Signal::cap(0.5),
                geo::Direction::North => neural::Signal::cap(1.0),
                geo::Direction::South => neural::Signal::cap(0.0),
            },
            Self::DirectionHorizontal => match world.being(index).direction() {
                geo::Direction::North | geo::Direction::South => neural::Signal::cap(0.5),
                geo::Direction::East => neural::Signal::cap(1.0),
                geo::Direction::West => neural::Signal::cap(0.0),
            },
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Output {
    Noop = 0,
    TurnLeft,
    TurnRight,
    Advance,
}

impl Output {
    fn from(index: usize) -> Self {
        // ALLOWED: This should come already clamped from the neural net
        // SAFTEY: This should come already clamped from the neural net
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index as u8)
        }
    }

    fn act(self, world: &mut world::World, index: usize) {
        match self {
            Self::Noop => println!("Noop"),
            Self::TurnLeft => {
                println!("Tunr left");
                world.being_mut(index).turn_left()
            }
            Self::TurnRight => {
                println!("Tunr right");
                world.being_mut(index).turn_right()
            }
            Self::Advance => {
                println!("Advancing");
                let direction = world.being(index).direction();
                world.advance(index, direction);
            }
        }
    }
}
