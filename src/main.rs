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

    let mut brain = neural::Brain::<Input, Output, 3, 0, 4>::new(
        [
            Input::Random,
            Input::DirectionVertical,
            Input::DirectionHorizontal,
        ],
        [
            Output::Noop,
            Output::TurnLeft,
            Output::TurnRight,
            Output::Advance,
        ],
    );

    brain.connect(0, 0, 0, 3, 1.0);

    for index in 0..world.count() {
        let outputs = brain.step(|input| input.retrieve(&world, index));
        for output in outputs {
            output.update(&mut world, index);
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
    Random,
    DirectionVertical,
    DirectionHorizontal,
}

impl Input {
    fn retrieve(&self, world: &world::World, index: usize) -> neural::Signal {
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
    Noop,
    TurnLeft,
    TurnRight,
    Advance,
}

impl Output {
    fn update(&self, world: &mut world::World, index: usize) {
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
