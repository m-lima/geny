use super::super::{Index, Simulation};
use crate::neural::{Stimulus, Synapse};

use crate::build_vec;

type Brain = super::super::super::neural::Brain<Input, Output, 8>;
type Axon = super::super::super::neural::Axon<Input, Output, 8>;

pub struct Mind {
    brain: Brain,
    genome: Genome,
}

impl Mind {
    pub fn random(synapses: u16, hidden_neurons: u8) -> Self {
        let genome = Genome::random(synapses, hidden_neurons);
        Self {
            brain: genome.build(),
            genome,
        }
    }

    #[inline]
    pub fn from(genome: Genome) -> Self {
        Self {
            brain: genome.build(),
            genome,
        }
    }

    #[inline]
    pub fn react(&mut self, simulation: &mut Simulation, index: Index) {
        self.brain
            .stimuli(|input| input.sense(simulation, index))
            .into_iter()
            .filter_map(|(out, stim)| out.spike(stim))
            .for_each(|(out, stim)| out.act(simulation, index, stim));
    }

    #[inline]
    pub fn genome(&self) -> &Genome {
        &self.genome
    }
}

pub struct Genome(Vec<Gene>);

impl Genome {
    fn random(synapses: u16, hidden_neurons: u8) -> Self {
        Self(build_vec!(
            || Gene::new(rand::random(), hidden_neurons),
            synapses
        ))
    }

    #[inline]
    pub fn signature(&self) -> u32 {
        self.0.iter().fold(0, |a, c| a ^ c.0)
    }

    pub fn combine(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(f, m)| if rand::random() { f } else { m })
                .copied()
                .collect(),
        )
    }

    pub fn mutate(&mut self, hidden_neurons: u8) {
        use rand::seq::SliceRandom;

        if let Some(mutation) = self.0.choose_mut(&mut rand::thread_rng()) {
            *mutation = mutation.mutate(hidden_neurons);
        }
    }

    fn build(&self) -> Brain {
        Brain::new(self.0.iter().map(Gene::build))
    }
}

// TODO: Control physical traits of the being as well
#[derive(Copy, Clone)]
struct Gene(u32);

impl Gene {
    const TWENTY_BITS: u32 = (1_u32 << 20) - 1;

    fn new(mut gene: u32, hidden_neurons: u8) -> Self {
        let (mut conn_type, mut input, mut output) = Self::dissect(gene);
        if hidden_neurons == 0 {
            conn_type = 0;
        }

        if conn_type & 0b10 == 0 {
            if input > Input::Random as u8 {
                input %= Input::Random as u8 + 1;
            }
        } else if input >= hidden_neurons {
            input %= hidden_neurons;
        }

        if conn_type == 0 || conn_type == 3 {
            if output > Output::Noop as u8 {
                output %= Output::Noop as u8 + 1;
            }
        } else if output >= hidden_neurons {
            output %= hidden_neurons;
        }

        gene &= Self::TWENTY_BITS;
        gene |= u32::from(conn_type) << 30;
        gene |= u32::from(input) << 25;
        gene |= u32::from(output) << 20;

        Self(gene)
    }

    fn mutate(self, hidden_neurons: u8) -> Self {
        let bit = 1 << (rand::random::<u8>() % 31_u8);
        Self::new(self.0 ^ bit, hidden_neurons)
    }

    // ALLOWED: The bitmask asserts proper range
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    fn dissect(mut gene: u32) -> (u8, u8, u8) {
        gene >>= 20;

        let output = gene as u8 & 0b1_1111;
        gene >>= 5;

        let input = gene as u8 & 0b1_1111;
        gene >>= 5;

        let conn_type = gene as u8 & 0b11;

        (conn_type, input, output)
    }

    // TODO: Test similarity (weigth < input < output < type)
    // ALLOWED: Makes for better chaining in `Genome::build`. (Either the ref happens in the
    // closure or here.. So..
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn build(&self) -> Axon {
        // ALLOWED: Mantissa is 23 bits, this is only 18
        #[allow(clippy::cast_precision_loss)]
        // 20 bits set to `1`
        // Divided by eight
        const REFERENCE: f32 = ((1_u32 << 17) - 1) as f32;

        // ALLOWED: Mantissa is 23 bits, this is only 20
        #[allow(clippy::cast_precision_loss)]
        // REFERENCE is divided by 8 so that we ultimately multiply `weight` by 8
        let synapse = (self.0 & Self::TWENTY_BITS) as f32 / REFERENCE - 4.;

        let (conn_type, input, output) = Self::dissect(self.0);

        match conn_type {
            0 => Axon::direct(
                Input::from(input),
                Output::from(output),
                Synapse::new(synapse),
            ),
            1 => Axon::into_hidden(Input::from(input), output as u8, Synapse::new(synapse)),
            2 => Axon::inter_hidden(input, output as u8, Synapse::new(synapse)),
            3 => Axon::from_hidden(input, Output::from(output), Synapse::new(synapse)),
            _ => unreachable!(),
        }
    }
}

// ALLOWED: It's the AI that uses it
#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Input {
    // TODO: Make `direction` a single input
    Direction,
    // DirectionVertical,
    // DirectionHorizontal,
    FoodDirection,
    FoodDistance,
    Unit,
    Random,
}

impl Input {
    fn from(index: u8) -> Self {
        // SAFETY: This private method is only called by a `Gene`
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index)
        }
    }

    fn sense(self, simulation: &Simulation, index: Index) -> Stimulus {
        match self {
            Self::Direction => {
                Stimulus::cap(simulation.boop(index).direction().as_rad() / std::f32::consts::TAU)
            }
            // Self::DirectionVertical => {
            //     Stimulus::cap(simulation.boop(index).direction().as_rad().sin() + 1. / 2.)
            // }
            // Self::DirectionHorizontal => {
            //     Stimulus::cap(simulation.boop(index).direction().as_rad().cos() + 1. / 2.)
            // }
            Self::FoodDirection => {
                let boop = simulation.boop(index);
                let coord = simulation.world.boop(index);
                let mut food = simulation
                    .fodder()
                    .map(|f| (f, f.distance(coord)))
                    // .filter_map(|f| {
                    //     let d = f.distance(coord);
                    //     if d <= 10. {
                    //         Some((f, d))
                    //     } else {
                    //         None
                    //     }
                    // })
                    .collect::<Vec<_>>();
                food.sort_unstable_by(|(_, d1), (_, d2)| {
                    d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal)
                });
                if let Some((f, _)) = food.first() {
                    Stimulus::cap(
                        (f.dir_from(coord) - boop.direction()).as_rad() / std::f32::consts::TAU,
                    )
                } else {
                    Stimulus::from(false)
                }
            }
            Self::FoodDistance => {
                let coord = simulation.world.boop(index);
                let mut food = simulation
                    .fodder()
                    .map(|f| f.distance(coord))
                    .collect::<Vec<_>>();
                food.sort_unstable_by(|d1, d2| {
                    d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal)
                });
                food.first().map_or(Stimulus::from(false), |d| {
                    Stimulus::cap(*d / f32::from(simulation.size()))
                })
            }
            Self::Unit => Stimulus::from(true),
            Self::Random => Stimulus::cap(rand::random::<f32>()),
        }
    }
}

// ALLOWED: It's the AI that uses it
#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Output {
    TurnLeft,
    TurnRight,
    Advance,
    Noop,
}

impl Output {
    fn from(index: u8) -> Self {
        // SAFETY: This private method is only called by a `Gene`
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index)
        }
    }

    fn spike(self, stimulus: Stimulus) -> Option<(Self, Stimulus)> {
        match self {
            Self::TurnLeft | Self::TurnRight | Self::Advance => Some((self, stimulus)),
            Self::Noop => None,
        }
    }

    fn act(self, simulation: &mut Simulation, index: Index, stimulus: Stimulus) {
        match self {
            Self::TurnLeft => {
                simulation.boop_mut(index).turn_left(stimulus.as_f32());
            }
            Self::TurnRight => {
                simulation.boop_mut(index).turn_right(stimulus.as_f32());
            }
            Self::Advance => {
                let direction = simulation.boop(index).direction();
                simulation
                    .world
                    .advance(index, stimulus.as_f32(), direction);
            }
            Self::Noop => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Gene, Input, Output};

    #[test]
    fn gene_is_safe() {
        for i in 0..8 {
            for _ in 0..8 {
                let gene = Gene::new(rand::random(), i);
                let (conn_type, input, output) = Gene::dissect(gene.0);

                if i == 0 {
                    assert!(conn_type == 0);
                }

                match conn_type {
                    0 => {
                        assert!(input <= Input::Random as u8, "{}", input,);
                        assert!(output <= Output::Noop as u8, "{}", output,);
                    }
                    1 => {
                        assert!(input <= Input::Random as u8, "{}", input,);
                        assert!(output < i);
                    }
                    2 => {
                        assert!(input < i);
                        assert!(output < i);
                    }
                    3 => {
                        assert!(input < i);
                        assert!(output <= Output::Noop as u8, "{}", output,);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn mutation() {
        let gene = Gene::new(rand::random(), 8);
        let mut reference: u32;

        for _ in 0..10 {
            reference = gene.0;
            gene.mutate(8);
            assert!((reference.max(gene.0) - reference.min(gene.0)) % 2 == 0);
        }
    }
}
