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

    println!("[c");
    for g in 0..100 {
        use rand::seq::SliceRandom;

        for i in 0..512_u16 {
            simulation.step();
            if i.trailing_ones() == 2 {
                println!("[68AGeneration: [37m{}[m", g);
                renderer::Renderer::render(&renderer::Terminal::<true>, &simulation);
                println!("{:.1}%", f32::from(i) / 5.12);
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        let survivors = simulation
            .indices()
            .filter_map(|i| {
                if simulation.world().being(i).x() <= 2 {
                    Some(simulation.genome(i))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if survivors.is_empty() {
            println!("EXTINCTION!!");
            break;
        }

        let mut children = Vec::with_capacity(beings as usize);

        let mut rng = rand::thread_rng();

        for _ in 0..beings {
            let father = survivors.choose(&mut rng).unwrap();
            let mother = survivors.choose(&mut rng).unwrap();
            children.push(father.combine(mother));
        }

        simulation.replace(children, hidden_neurons);
    }

    Ok(())
}
