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

    let grid = world::World::new(size, count);

    renderer::Renderer::render(&renderer::Terminal, &grid);

    Ok(())
}

// #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
// enum Input {
//     Random,
//     DirectionVertical,
//     DirectionHorizontal,
// }

// impl Input {
//     fn spike(&self, world: &World, index: usize) -> signal::Half {
//         use signal::Signal;

//         match self {
//             Self::Random => signal::Half::cap(rand::random::<f32>()),
//             Self::DirectionVertical => match world.being(index).direction() {
//                 Direction::East | Direction::West => signal::Half::cap(0.5),
//                 Direction::North => signal::Half::cap(1.0),
//                 Direction::South => signal::Half::cap(0.0),
//             },
//             Self::DirectionHorizontal => match world.being(index).direction() {
//                 Direction::North | Direction::South => signal::Half::cap(0.5),
//                 Direction::East => signal::Half::cap(1.0),
//                 Direction::West => signal::Half::cap(0.0),
//             },
//         }
//     }
// }

// #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
// enum Output {
//     Noop,
//     TurnLeft,
//     TurnRight,
//     Advance,
// }

// impl Output {
//     fn spike(&self, world: &mut World, index: usize) {
//         match self {
//             Self::Noop => {}
//             Self::TurnLeft => world.being_mut(index).turn_left(),
//             Self::TurnRight => world.being_mut(index).turn_right(),
//             Self::Advance => {
//                 let direction = world.being(index).direction();
//                 world.advance(index, direction);
//             }
//         }
//     }
// }
