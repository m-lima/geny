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

    let mut brain = neural::Brain::<State<'_>, Input, Output, 3, 0, 4>::new(
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

    for index in 0..world.count() {
        let mut state = State {
            world: &mut world,
            index,
        };
        brain.step(&mut state);
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

impl neural::Input<State<'_>> for Input {
    fn update(&self, state: &State<'_>) -> neural::Signal {
        match self {
            Self::Random => neural::Signal::cap(rand::random::<f32>()),
            Self::DirectionVertical => match state.world.being(state.index).direction() {
                geo::Direction::East | geo::Direction::West => neural::Signal::cap(0.5),
                geo::Direction::North => neural::Signal::cap(1.0),
                geo::Direction::South => neural::Signal::cap(0.0),
            },
            Self::DirectionHorizontal => match state.world.being(state.index).direction() {
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

impl neural::Output<State<'_>> for Output {
    fn update(&self, state: &mut State<'_>) {
        match self {
            Self::Noop => {}
            Self::TurnLeft => state.world.being_mut(state.index).turn_left(),
            Self::TurnRight => state.world.being_mut(state.index).turn_right(),
            Self::Advance => {
                let direction = state.world.being(state.index).direction();
                state.world.advance(state.index, direction);
            }
        }
    }
}
