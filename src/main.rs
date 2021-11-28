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

    renderer::Renderer::render(&renderer::Terminal::<true>, &world);

    // let mut brain = neural::Brain::<3, 0, 4>::new::<4>(vec![
    //     neural::Gene::new(
    //         true,
    //         true,
    //         Input::Random as usize,
    //         Output::Advance as usize,
    //         neural::Signal::cap(1.0),
    //     ),
    //     neural::Gene::new(
    //         true,
    //         true,
    //         Input::DirectionHorizontal as usize,
    //         Output::TurnLeft as usize,
    //         neural::Signal::cap(1.0),
    //     ),
    //     neural::Gene::new(
    //         true,
    //         true,
    //         Input::DirectionVertical as usize,
    //         Output::TurnLeft as usize,
    //         neural::Signal::cap(0.5),
    //     ),
    // ]);

    for _ in 0..10 {
        for index in 0..world.count() {
            let outputs = brain.step(|input| Input::from(input).sense(&world, index));
            for (output, signal) in outputs
                .iter()
                .enumerate()
                .map(|(i, signal)| (Output::from(i), signal))
            {
                if rand::random::<f32>() < signal.as_f32() {
                    output.act(&mut world, index);
                }
            }
        }

        renderer::Renderer::render(&renderer::Terminal::<true>, &world);
    }

    Ok(())
}
