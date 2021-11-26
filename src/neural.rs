use super::geo::Direction;
use super::world::World;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Input {
    Random,
    DirectionVertical,
    DirectionHorizontal,
}

impl Input {
    fn spike(&self, world: &World, index: usize) -> signal::Half {
        use signal::Signal;

        match self {
            Self::Random => signal::Half::cap(rand::random::<f32>()),
            Self::DirectionVertical => match world.being(index).direction() {
                Direction::East | Direction::West => signal::Half::cap(0.5),
                Direction::North => signal::Half::cap(1.0),
                Direction::South => signal::Half::cap(0.0),
            },
            Self::DirectionHorizontal => match world.being(index).direction() {
                Direction::North | Direction::South => signal::Half::cap(0.5),
                Direction::East => signal::Half::cap(1.0),
                Direction::West => signal::Half::cap(0.0),
            },
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Output {
    Noop,
    TurnLeft,
    TurnRight,
    Advance,
}

impl Output {
    fn spike(&self, world: &mut World, index: usize) {
        match self {
            Self::Noop => {}
            Self::TurnLeft => world.being_mut(index).turn_left(),
            Self::TurnRight => world.being_mut(index).turn_right(),
            Self::Advance => {
                let direction = world.being(index).direction();
                world.advance(index, direction);
            }
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Axon {
    Input(Input),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Dentrite {
    Output(Output),
}

#[derive(Clone, PartialOrd, PartialEq, Debug)]
struct Synapsis {
    axon: Axon,
    dentrite: Dentrite,
    amplifier: signal::Amplifier<4>,
}

struct Brain {
    synapses: std::collections::HashMap<Axon, Vec<Synapsis>>,
}

impl Brain {
    pub fn new(mut synapses: Vec<Synapsis>) -> Self {
        let synapses = synapses
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut a, c| {
                a.entry(c.axon).or_insert(vec![]).push(c);
                a
            });
        Self { synapses }
    }

    pub fn fire(&self, world: &World, index: usize) {}
}

mod signal {
    pub trait Signal {
        fn cap(value: f32) -> Self;
        fn as_f32(&self) -> f32;
    }

    pub struct Full(f32);

    impl Signal for Full {
        fn cap(value: f32) -> Self {
            Self(value.min(1.0).max(-1.0))
        }

        fn as_f32(&self) -> f32 {
            self.0
        }
    }

    pub struct Half(f32);

    impl Signal for Half {
        fn cap(value: f32) -> Self {
            Self(value.min(1.0).max(0.0))
        }

        fn as_f32(&self) -> f32 {
            self.0
        }
    }

    #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
    pub struct Amplifier<const AMPLITUDE: u8>(f32);

    impl<const AMPLITUDE: u8> Amplifier<AMPLITUDE> {
        fn amplify<S: Signal>(signal: S) -> S {
            S::cap(signal.as_f32())
        }
    }
}
