use super::super::geo::Direction;
use super::super::neural::{Axon, Stimuli, Synapse};
use super::{Index, Simulation};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Input {
    Random = 0,
    DirectionVertical,
    DirectionHorizontal,
}

impl Input {
    fn from(index: u32) -> Self {
        // ALLOWED: This should come already clamped from the gene
        // SAFTEY: This should come already clamped from the gene
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index as u8)
        }
    }

    fn sense<const H: u8, const S: usize>(
        self,
        simulation: &Simulation<H, S>,
        index: Index,
    ) -> Stimuli {
        match self {
            Self::Random => Stimuli::cap(rand::random::<f32>()),
            Self::DirectionVertical => match simulation.being(index).direction() {
                Direction::East | Direction::West => Stimuli::half(),
                Direction::North => true.into(),
                Direction::South => false.into(),
            },
            Self::DirectionHorizontal => match simulation.being(index).direction() {
                Direction::North | Direction::South => Stimuli::half(),
                Direction::East => true.into(),
                Direction::West => false.into(),
            },
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Output {
    Noop = 0,
    TurnLeft,
    TurnRight,
    Advance,
}

impl Output {
    fn from(index: u32) -> Self {
        // ALLOWED: This should come already clamped from the gene
        // SAFTEY: This should come already clamped from the gene
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Self>(index as u8)
        }
    }

    fn act<const H: u8, const S: usize>(self, simulation: &mut Simulation<H, S>, index: Index) {
        match self {
            Self::Noop => println!("Noop"),
            Self::TurnLeft => {
                println!("Tunr left");
                simulation.being_mut(index).turn_left();
            }
            Self::TurnRight => {
                println!("Tunr right");
                simulation.being_mut(index).turn_right();
            }
            Self::Advance => {
                println!("Advancing");
                let direction = simulation.being(index).direction();
                simulation.world.advance(index, direction);
            }
        }
    }
}

pub struct Genome<const H: u8, const S: usize>([Gene<H>; S]);

impl<const H: u8, const S: usize> Genome<H, S> {
    // fn build(&self) -> Brain<Input, Output> {
    //     Brain::new(self.0.iter().map(Gene::to_axon))
    // }

    fn derive(&self, other: &Self) -> Self {
        // TODO: This
        unimplemented!()
        // Self(
        //     self.0
        //         .iter()
        //         .zip(other.0.iter())
        //         .map(|(a, b)| if rand::random() { a } else { b })
        //         .copied()
        //         .collect(),
        // )
    }
}

#[derive(Copy, Clone)]
struct Gene<const H: u8>(u32);

impl<const H: u8> Gene<H> {
    pub fn random() -> Self {
        Self(rand::random())
    }

    pub fn mutate(&mut self) {
        let bit = 1 << (rand::random::<u8>() % 31_u8);
        self.0 ^ bit;
    }

    // TODO: Test similarity (weigth < input < output < type)
    fn to_axon(&self) -> Axon<Input, Output, H> {
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
            1 => Axon::into_hidden(Input::from(input), output as u8, Synapse::new(synapse)),
            2 => Axon::inter_hidden(input as u8, output as u8, Synapse::new(synapse)),
            3 => Axon::from_hidden(input as u8, Output::from(output), Synapse::new(synapse)),
            _ => unreachable!(),
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
