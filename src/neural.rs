pub use signal::Signal;
use signal::{Aggregator, Amplifier, Tangential};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Layer {
    Input,
    Internal,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NeuronRef {
    layer: Layer,
    index: usize,
}

struct Dentrite {
    axon: NeuronRef,
    synapse: Amplifier,
}

struct Input {
    latch: Signal,
    visited: bool,
}

impl Input {
    fn new() -> Self {
        Self {
            latch: Signal::new(),
            visited: false,
        }
    }
}

struct Output {
    dentrites: Vec<Dentrite>,
}

impl Output {
    fn new() -> Self {
        Self { dentrites: vec![] }
    }
}

struct Internal {
    dentrites: Vec<Dentrite>,
    latch: Signal,
    visited: bool,
}

impl Internal {
    fn new() -> Self {
        Self {
            dentrites: vec![],
            latch: Signal::new(),
            visited: false,
        }
    }
}

pub struct Gene {
    input: bool,
    output: bool,
    in_index: usize,
    out_index: usize,
    amplifier: Signal,
}

impl Gene {
    pub fn new(
        input: bool,
        output: bool,
        in_index: usize,
        out_index: usize,
        amplifier: Signal,
    ) -> Self {
        Self {
            input,
            output,
            in_index,
            out_index,
            amplifier,
        }
    }
}

pub struct Brain<const INPUTS: usize, const INTERNALS: usize, const OUTPUTS: usize> {
    // TODO: Consider using Vec here, so that we only have instances that are actually used by the
    // genome
    // TODO: Can the brain not have any templates and let the genome drive the layout?
    inputs: [Input; INPUTS],
    internals: [Internal; INTERNALS],
    outputs: [Output; OUTPUTS],
}

impl<const INPUTS: usize, const INTERNALS: usize, const OUTPUTS: usize>
    Brain<INPUTS, INTERNALS, OUTPUTS>
{
    pub fn new<const A: u8>(genome: Vec<Gene>) -> Self {
        let mut brain = Self {
            inputs: [0; INPUTS].map(|_| Input::new()),
            internals: [0; INTERNALS].map(|_| Internal::new()),
            outputs: [0; OUTPUTS].map(|_| Output::new()),
        };

        for gene in genome {
            let dentrite = Dentrite {
                axon: NeuronRef {
                    layer: if gene.input {
                        Layer::Input
                    } else {
                        Layer::Internal
                    },
                    index: if gene.input {
                        gene.in_index % INPUTS
                    } else {
                        gene.in_index % INTERNALS
                    },
                },
                synapse: Amplifier::new(gene.amplifier.as_f32() * f32::from(A)),
            };

            if gene.output {
                brain.outputs[gene.out_index % OUTPUTS]
                    .dentrites
                    .push(dentrite);
            } else {
                brain.internals[gene.out_index % INTERNALS]
                    .dentrites
                    .push(dentrite);
            }
        }

        brain
    }

    pub fn step(&mut self, input: impl Copy + Fn(usize) -> Signal) -> [Signal; OUTPUTS] {
        self.inputs.iter_mut().for_each(|i| i.visited = false);
        self.internals.iter_mut().for_each(|i| i.visited = false);

        let mut output = [Signal::new(); OUTPUTS];

        for (i, o) in self.outputs.iter().enumerate() {
            output[i] = <Tangential as Aggregator>::aggregate(o.dentrites.iter().map(|d| {
                d.synapse * Self::update(&mut self.inputs, &mut self.internals, d.axon, input)
            }));
        }

        output
    }

    fn update(
        inputs: &mut [Input; INPUTS],
        internals: &mut [Internal; INTERNALS],
        neuron_ref: NeuronRef,
        input: impl Copy + Fn(usize) -> Signal,
    ) -> Signal {
        match neuron_ref.layer {
            Layer::Input => {
                let neuron = &mut inputs[neuron_ref.index];
                if !neuron.visited {
                    neuron.visited = true;
                    neuron.latch = input(neuron_ref.index);
                }
                neuron.latch
            }
            Layer::Internal => {
                let neuron: *mut Internal = &mut internals[neuron_ref.index];
                // SAFETY: Safe because we never modify the list nor do we revisit a node
                unsafe {
                    if !(*neuron).visited {
                        (*neuron).visited = true;
                        (*neuron).latch =
                            <Tangential as Aggregator>::aggregate((*neuron).dentrites.iter().map(
                                |d| d.synapse * Self::update(inputs, internals, d.axon, input),
                            ));
                    }
                    (*neuron).latch
                }
            }
        }
    }
}

mod signal {
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub struct Signal(f32);

    impl Signal {
        pub fn new() -> Self {
            Self(0.0)
        }

        pub fn cap(value: f32) -> Self {
            Self(value.min(1.0).max(-1.0))
        }

        pub fn as_f32(self) -> f32 {
            self.0
        }
    }

    // TODO: When creating new dentrites, this value should be capped
    #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
    pub struct Amplifier(f32);

    impl Amplifier {
        pub fn new(value: f32) -> Self {
            Self(value)
        }
    }

    impl std::ops::Mul<Signal> for Amplifier {
        type Output = Signal;

        fn mul(self, rhs: Signal) -> Self::Output {
            Signal::cap(rhs.as_f32() * self.0)
        }
    }

    pub trait Aggregator {
        fn aggregate(inputs: impl Iterator<Item = Signal>) -> Signal;
    }

    pub struct Linear;
    impl Aggregator for Linear {
        fn aggregate(inputs: impl Iterator<Item = Signal>) -> Signal {
            let sum = inputs
                .enumerate()
                .fold((0, 0.0), |a, c| (c.0, a.1 + c.1.as_f32()));
            // ALLOWED: Because there cannot be that many synapses (not enough neurons to
            // saturate 23 bits)
            #[allow(clippy::cast_precision_loss)]
            Signal::cap(sum.1 / sum.0 as f32)
        }
    }

    pub struct Tangential;
    impl Aggregator for Tangential {
        fn aggregate(inputs: impl Iterator<Item = Signal>) -> Signal {
            Signal::cap(inputs.fold(0.0, |a, c| a + c.as_f32()).tanh())
        }
    }
}
