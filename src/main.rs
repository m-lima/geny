#![deny(warnings, clippy::pedantic, rust_2018_idioms, rust_2021_compatibility)]

#[macro_export]
macro_rules! build_vec {
    ($builder:expr, $size:expr) => {{
        let builder = $builder;
        (0..$size).map(|_| builder()).collect()
    }};
}

mod engine;
mod neural;
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

    let days = args
        .next()
        .ok_or(anyhow::anyhow!("No days per generation provided"))?
        .parse()?;

    let synapses = args
        .next()
        .ok_or(anyhow::anyhow!("No synapse count provided"))?
        .parse()?;

    let hidden_neurons = args
        .next()
        .ok_or(anyhow::anyhow!("No hidden neuron count provided"))?
        .parse()?;

    let terminal = args.next().map(|s| s.trim() == "t").is_some();

    let simulation = sim::Simulation::new(size, beings, synapses, hidden_neurons);

    if terminal {
        use engine::Engine;
        let terminal = engine::Terminal::<true, true>;
        terminal.start(simulation, days);
    } else {
        use engine::Engine;
        let quad = engine::Quad::new(macroquad::window::Conf {
            window_title: String::from("Geny"),
            window_width: 800,
            window_height: 800,
            window_resizable: false,
            ..macroquad::window::Conf::default()
        });
        quad.start(simulation, days);
    }

    Ok(())
}
