#![allow(dead_code)]

mod being;
mod geo;
mod neural;
mod renderer;
mod world;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let size = args
        .next()
        .ok_or(anyhow::anyhow!("No size provided"))?
        .parse()?;

    let count = args
        .next()
        .ok_or(anyhow::anyhow!("No count provided"))?
        .parse()?;

    let mut world = world::World::new(size, count);

    renderer::Renderer::render(&renderer::Terminal, &world);

    let mut brain = neural::Brain::<3, 0, 4>::new::<4>(vec![neural::Gene::new(
        true,
        true,
        Input::Random as usize,
        Output::Advance as usize,
        neural::Signal::cap(1.0),
    )]);

    // brain.connect(0, 0, 0, 3, 1.0);

    for index in 0..world.count() {
        let outputs = brain.step(|input| Input::from(input).sense(&world, index));
        for (output, _) in outputs
            .iter()
            .enumerate()
            .map(|(i, signal)| (Output::from(i), signal))
        {
            output.act(&mut world, index);
        }
    }

    renderer::Renderer::render(&renderer::Terminal, &world);

    Ok(())
}

struct State<'a> {
    world: &'a mut world::World,
    index: usize,
}

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
