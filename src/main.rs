#![allow(dead_code)]

#[macro_export]
macro_rules! build_vec {
    ($builder:expr, $size:expr) => {{
        let builder = $builder;
        (0..$size).map(|_| builder()).collect()
    }};
}

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

    while simulation.step() {
        println!("[67A");
        // println!("[68AGeneration: [37m{}[m", g);
        renderer::Renderer::render(&renderer::Terminal::<true>, &simulation);
        // std::thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(())
}
