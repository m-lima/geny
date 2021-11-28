use super::super::geo::Direction;
use super::super::neural::{Axon, Brain, Stimulus, Synapse};
use super::{Index, Simulation};

pub struct Genome(Vec<Gene>);

impl Genome {
    pub fn random(len: u16) -> Self {
        Self((0..len).map(|_| Gene::random()).collect())
    }

    pub fn id(&self) -> u32 {
        self.0.iter().fold(0, |a, c| a ^ c.0)
    }

    pub fn combine(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| if rand::random() { a } else { b })
                .copied()
                .collect(),
        )
    }

    pub fn to_brain(&self, hidden_neurons: u8) -> Brain<Input, Output, { super::HIDDEN_NEURONS }> {
        Brain::new(self.0.iter().map(|g| g.to_axon(hidden_neurons)))
    }
}

// TODO: Control physical traits of the being as well
#[derive(Copy, Clone)]
struct Gene(u32);

impl Gene {
    fn random() -> Self {
        Self(rand::random())
    }

    fn mutate(&mut self) {
        let bit = 1 << (rand::random::<u8>() % 31_u8);
        self.0 ^= bit;
    }

    // TODO: Test similarity (weigth < input < output < type)
    fn to_axon(self, hidden_neurons: u8) -> Axon<Input, Output, { super::HIDDEN_NEURONS }> {
        // ALLOWED: Mantissa is 23 bits, this is only 18
        #[allow(clippy::cast_precision_loss)]
        // 20 bits set to `1`
        // Divided by four
        const REFERENCE: f32 = ((1_u32 << 18) - 1) as f32;

        const TWENTY_BITS: u32 = (1_u32 << 20) - 1;

        let mut value = self.0;

        // ALLOWED: Mantissa is 23 bits, this is only 20
        #[allow(clippy::cast_precision_loss)]
        // REFERENCE is divided by 4 so that we ultimately multiply `weight` by 4
        let synapse = (value & TWENTY_BITS) as f32 / REFERENCE;
        value >>= 20;

        let input = value & 0b1_1111;
        value >>= 5;

        let output = value & 0b1_1111;
        value >>= 5;

        let conn_type = value & 0b11;

        // ALLOWED: Bitmask above already keeps values in range
        #[allow(clippy::cast_possible_truncation)]
        match conn_type {
            0 => Axon::direct(
                Input::from(input),
                Output::from(output),
                Synapse::new(synapse),
            ),
            1 => Axon::into_hidden(
                Input::from(input),
                output as u8 % hidden_neurons,
                Synapse::new(synapse),
            ),
            2 => Axon::inter_hidden(
                input as u8 % hidden_neurons,
                output as u8 % hidden_neurons,
                Synapse::new(synapse),
            ),
            3 => Axon::from_hidden(
                input as u8 % hidden_neurons,
                Output::from(output),
                Synapse::new(synapse),
            ),
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Input {
    DirectionVertical,
    DirectionHorizontal,
    Random,
}

impl Input {
    fn from(index: u32) -> Self {
        // ALLOWED: The modulus asserts proper range
        // SAFETY: The modulus asserts proper range
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>((index % (Self::Random as u32) + 1) as u8)
        }
    }

    pub(super) fn sense(self, simulation: &Simulation, index: Index) -> Stimulus {
        match self {
            Self::Random => Stimulus::cap(rand::random::<f32>()),
            Self::DirectionVertical => match simulation.being(index).direction() {
                Direction::East | Direction::West => Stimulus::half(),
                Direction::North => true.into(),
                Direction::South => false.into(),
            },
            Self::DirectionHorizontal => match simulation.being(index).direction() {
                Direction::North | Direction::South => Stimulus::half(),
                Direction::East => true.into(),
                Direction::West => false.into(),
            },
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Output {
    TurnLeft,
    TurnRight,
    Advance,
    Noop,
}

impl Output {
    fn from(index: u32) -> Self {
        // ALLOWED: The modulus asserts proper range
        // SAFETY: The modulus asserts proper range
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>((index % (Self::Noop as u32) + 1) as u8)
        }
    }

    pub(super) fn act(self, simulation: &mut Simulation, index: Index) {
        match self {
            Self::Noop => {}
            Self::TurnLeft => {
                simulation.being_mut(index).turn_left();
            }
            Self::TurnRight => {
                simulation.being_mut(index).turn_right();
            }
            Self::Advance => {
                let direction = simulation.being(index).direction();
                simulation.world.advance(index, direction);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Gene;

    #[test]
    fn mutation() {
        let mut gene = Gene::random();
        let mut reference: u32;

        for _ in 0..10 {
            reference = gene.0;
            gene.mutate();
            assert!((reference.max(gene.0) - reference.min(gene.0)) % 2 == 0);
        }
    }
}
