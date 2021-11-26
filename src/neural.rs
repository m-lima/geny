#[derive(Copy, Clone, Eq, PartialEq)]
enum Visitability {
    Unvisited,
    Visiting,
    Visited,
}

struct Dentrite<'a, const A: u8> {
    // TODO: This may cause problems. If that is the case, we can use a Vec arena index
    axon: &'a mut Neuron<'a, A>,
    synapse: signal::Amplifier<A>,
}

struct Neuron<'a, const A: u8> {
    dentrites: Vec<Dentrite<'a, A>>,
    aggregator: signal::Aggregator,
    latch: signal::Full,
    visitability: Visitability,
}

impl<'a, const A: u8> Neuron<'a, A> {
    fn update(&mut self) -> signal::Full {
        if self.visitability == Visitability::Unvisited {
            self.visitability = Visitability::Visiting;

            self.latch = self.aggregator.aggregate(
                self.dentrites
                    .iter_mut()
                    .map(|d| d.synapse * d.axon.update()),
            );

            self.visitability = Visitability::Visited;
        }

        self.latch
    }
}

mod signal {
    pub trait Signal {
        fn cap(value: f32) -> Self;
        fn as_f32(&self) -> f32;
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub struct Full(f32);

    impl Signal for Full {
        fn cap(value: f32) -> Self {
            Self(value.min(1.0).max(-1.0))
        }

        fn as_f32(&self) -> f32 {
            self.0
        }
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub struct Half(f32);

    impl Signal for Half {
        fn cap(value: f32) -> Self {
            Self(value.min(1.0).max(0.0))
        }

        fn as_f32(&self) -> f32 {
            self.0
        }
    }

    #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
    pub struct Amplifier<const AMPLITUDE: u8>(f32);

    impl<const AMPLITUDE: u8> std::ops::Mul<Full> for Amplifier<AMPLITUDE> {
        type Output = Full;

        fn mul(self, rhs: Full) -> Self::Output {
            Full::cap(rhs.as_f32() * AMPLITUDE as f32)
        }
    }

    impl<const AMPLITUDE: u8> std::ops::Mul<Half> for Amplifier<AMPLITUDE> {
        type Output = Half;

        fn mul(self, rhs: Half) -> Self::Output {
            Half::cap(rhs.as_f32() * AMPLITUDE as f32)
        }
    }

    impl<const AMPLITUDE: u8> Amplifier<AMPLITUDE> {
        fn amplify<S: Signal>(signal: S) -> S {
            S::cap(signal.as_f32())
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Aggregator {
        Linear,
        Tangential,
        Exponential,
    }

    impl Aggregator {
        pub fn aggregate(&self, inputs: impl Iterator<Item = Full>) -> Full {
            unimplemented!();
        }
    }
}
