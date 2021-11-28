#![allow(dead_code)]

// TODO: Start with a "be at" for exclusion. Later add food. Later add reproduction without
// generations. Later add preying

mod geo;
mod neural;
mod renderer;
mod sim;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let size = args
        .next()
        .ok_or(anyhow::anyhow!("No size provided"))?
        .parse()?;

    let beings = args
        .next()
        .ok_or(anyhow::anyhow!("No being count provided"))?
        .parse()?;

    let synapses = args
        .next()
        .ok_or(anyhow::anyhow!("No synapse count provided"))?
        .parse()?;

    let hidden_neurons = args
        .next()
        .ok_or(anyhow::anyhow!("No hidden neuron count provided"))?
        .parse()?;

    let mut simulation = sim::Simulation::new(size, beings, synapses, hidden_neurons);

    renderer::Renderer::render(&renderer::Terminal::<true>, &simulation);

    for i in 0..1024_u16 {
        println!("c");
        simulation.step();
        renderer::Renderer::render(&renderer::Terminal::<true>, &simulation);
        println!("{:.1}%", f32::from(i) / 10.24);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(())
}
