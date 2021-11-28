mod being;
mod gene;
mod world;

pub use being::Being;
pub use gene::{Genome, Input, Output};
pub use world::World;

const HIDDEN_NEURONS: u8 = 8;

pub struct Simulation {
    world: World,
    beings: Vec<Being>,
    genomes: Vec<Genome>,
}

impl Simulation {
    pub fn new(size: u8, beings: u16, synapses: u16, hidden_neurons: u8) -> Self {
        let genomes = (0..beings)
            .map(|_| Genome::random(synapses))
            .collect::<Vec<_>>();
        Self {
            world: World::new(size, beings),
            beings: genomes
                .iter()
                .map(|g| g.to_brain(hidden_neurons))
                .map(Being::new)
                .collect(),
            genomes,
        }
    }

    pub fn step(&mut self) {
        // TODO: Randomize order
        // TODO: Make a Cow of the modifiable values
        for index in self.indices() {
            let being: *mut Being = self.being_mut(index);
            // SAFETY: `being` never gets dropped or moved
            let outputs = unsafe { (*being).brain() }.stimuli(|input| input.sense(self, index));
            for output in outputs.iter().filter_map(|o| {
                if rand::random::<f32>() < o.1.as_f32() {
                    Some(o.0)
                } else {
                    None
                }
            }) {
                output.act(self, index);
            }
        }
    }

    // TODO: This is garbage!
    pub fn replace(&mut self, population: Vec<Genome>, hidden_neurons: u8) {
        assert!(self.genomes.len() == population.len());
        self.genomes = population;
        self.world.shuffle();
        self.beings = self
            .genomes
            .iter()
            .map(|g| g.to_brain(hidden_neurons))
            .map(Being::new)
            .collect();
    }

    #[inline]
    pub fn world(&self) -> &World {
        &self.world
    }

    #[inline]
    pub fn indices(&self) -> impl Iterator<Item = Index> {
        (0..self.beings.len()).map(Index)
    }

    #[inline]
    pub fn being(&self, index: Index) -> &Being {
        unsafe { self.beings.get_unchecked(index.0) }
    }

    #[inline]
    pub fn genome(&self, index: Index) -> &Genome {
        unsafe { self.genomes.get_unchecked(index.0) }
    }

    #[inline]
    fn being_mut(&mut self, index: Index) -> &mut Being {
        unsafe { self.beings.get_unchecked_mut(index.0) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Index(usize);
