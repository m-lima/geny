pub use signal::Signal;
use signal::{Aggregator, Amplifier, Tangential};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Layer {
    Input,
    Hidden,
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

trait Neuron<I: Copy + Eq> {
    fn index(&self) -> I;
}

trait OutNeuron<I: Copy + Eq>: Neuron<I> {
    fn dentrites(&self) -> &Vec<Dentrite>;
    fn dentrites_mut(&mut self) -> &mut Vec<Dentrite>;
}

struct Input<I: Copy + Eq> {
    index: I,
    latch: Signal,
    visited: bool,
}

impl<I: Copy + Eq> Input<I> {
    fn new(index: I) -> Self {
        Self {
            index,
            latch: Signal::new(),
            visited: false,
        }
    }
}

impl<I: Copy + Eq> Neuron<I> for Input<I> {
    #[inline]
    fn index(&self) -> I {
        self.index
    }
}

struct Hidden<I: Copy + Eq> {
    index: I,
    dentrites: Vec<Dentrite>,
    latch: Signal,
    visited: bool,
}

impl<I: Copy + Eq> Hidden<I> {
    fn new(index: I) -> Self {
        Self {
            index,
            dentrites: vec![],
            latch: Signal::new(),
            visited: false,
        }
    }
}

impl<I: Copy + Eq> Neuron<I> for Hidden<I> {
    #[inline]
    fn index(&self) -> I {
        self.index
    }
}

impl<I: Copy + Eq> OutNeuron<I> for Hidden<I> {
    #[inline]
    fn dentrites(&self) -> &Vec<Dentrite> {
        &self.dentrites
    }

    #[inline]
    fn dentrites_mut(&mut self) -> &mut Vec<Dentrite> {
        &mut self.dentrites
    }
}

struct Output<I: Copy + Eq> {
    index: I,
    dentrites: Vec<Dentrite>,
}

impl<I: Copy + Eq> Output<I> {
    fn new(index: I) -> Self {
        Self {
            index,
            dentrites: vec![],
        }
    }
}

impl<I: Copy + Eq> Neuron<I> for Output<I> {
    #[inline]
    fn index(&self) -> I {
        self.index
    }
}

impl<I: Copy + Eq> OutNeuron<I> for Output<I> {
    #[inline]
    fn dentrites(&self) -> &Vec<Dentrite> {
        &self.dentrites
    }

    #[inline]
    fn dentrites_mut(&mut self) -> &mut Vec<Dentrite> {
        &mut self.dentrites
    }
}

#[derive(Hash)]
pub enum Gene<Input: Copy + Eq, Hidden: Copy + Eq, Output: Copy + Eq> {
    Direct {
        input: Input,
        output: Output,
        amplifier: Signal,
    },
    IntoHidden {
        input: Input,
        output: Hidden,
        amplifier: Signal,
    },
    InterHidden {
        input: Hidden,
        output: Hidden,
        amplifier: Signal,
    },
    FromHidden {
        input: Hidden,
        output: Output,
        amplifier: Signal,
    },
}

pub struct Brain<I: Copy + Eq, H: Copy + Eq, O: Copy + Eq> {
    inputs: Vec<Input<I>>,
    hiddens: Vec<Hidden<H>>,
    outputs: Vec<Output<O>>,
}

impl<I: Copy + Eq, H: Copy + Eq, O: Copy + Eq> Brain<I, H, O> {
    pub fn new<const A: u8>(genome: Vec<Gene<I, H, O>>) -> Self {
        let inputs: Vec<Input<I>> = vec![];
        let hiddens: Vec<Hidden<H>> = vec![];
        let outputs: Vec<Output<O>> = vec![];

        for gene in genome {
            match gene {
                Gene::Direct {
                    input,
                    output,
                    amplifier,
                } => {
                    Self::make_synapse(
                        input,
                        output,
                        amplifier.as_f32() * f32::from(A),
                        &mut inputs,
                        &mut outputs,
                        Input::new,
                        Output::new,
                    );
                }
                Gene::IntoHidden {
                    input,
                    output,
                    amplifier,
                } => {
                    Self::make_synapse(
                        input,
                        output,
                        amplifier.as_f32() * f32::from(A),
                        &mut inputs,
                        &mut hiddens,
                        Input::new,
                        Hidden::new,
                    );
                }
                Gene::InterHidden {
                    input,
                    output,
                    amplifier,
                } => {
                    Self::make_synapse(
                        input,
                        output,
                        amplifier.as_f32() * f32::from(A),
                        &mut hiddens,
                        &mut hiddens,
                        Hidden::new,
                        Hidden::new,
                    );
                }
                Gene::FromHidden {
                    input,
                    output,
                    amplifier,
                } => {
                    Self::make_synapse(
                        input,
                        output,
                        amplifier.as_f32() * f32::from(A),
                        &mut hiddens,
                        &mut outputs,
                        Hidden::new,
                        Output::new,
                    );
                }
            }
        }

        Self {
            inputs,
            hiddens,
            outputs,
        }
    }

    fn make_synapse<
        In: Copy + Eq,
        Out: Copy + Eq,
        NIn: Neuron<In>,
        NOut: OutNeuron<Out>,
        BIn: Fn(In) -> NIn,
        BOut: Fn(Out) -> NOut,
    >(
        input: In,
        output: Out,
        amplifier: f32,
        inputs: &mut Vec<NIn>,
        outputs: &mut Vec<NOut>,
        in_builder: BIn,
        out_builder: BOut,
    ) {
        let input_index = if let Some(idx) = inputs
            .iter()
            .enumerate()
            .find(|(_, i)| i.index() == input)
            .map(|(i, _)| i)
        {
            idx
        } else {
            inputs.push(in_builder(input));
            inputs.len() - 1
        };

        let dentrite = Dentrite {
            axon: NeuronRef {
                layer: Layer::Input,
                index: input_index,
            },
            synapse: Amplifier::new(amplifier),
        };

        if let Some(output) = outputs.iter().find(|i| i.index() == output) {
            output.dentrites_mut().push(dentrite);
        } else {
            let output = out_builder(output);
            output.dentrites_mut().push(dentrite);
            outputs.push(output);
        }
    }

    pub fn step(&mut self, input: impl Copy + Fn(usize) -> Signal) -> Vec<(O, Signal)> {
        self.clear_visits();

        let mut output = Vec::with_capacity(self.outputs.len());

        for (i, o) in self.outputs.iter().enumerate() {
            let signal = <Tangential as Aggregator>::aggregate(o.dentrites.iter().map(|d| {
                d.synapse * Self::update(&mut self.inputs, &mut self.hiddens, d.axon, input)
            }));
            output.push((o.index, signal));
        }

        output
    }

    fn clear_visits(&mut self) {
        self.inputs.iter_mut().for_each(|i| i.visited = false);
        self.hiddens.iter_mut().for_each(|i| i.visited = false);
    }

    fn update(
        inputs: &mut Vec<Input<I>>,
        hiddens: &mut Vec<Hidden<H>>,
        neuron_ref: NeuronRef,
        input: impl Copy + Fn(usize) -> Signal,
    ) -> Signal {
        match neuron_ref.layer {
            Layer::Input => {
                // SAFETY: References are never out of bounds
                let neuron = unsafe { inputs.get_unchecked_mut(neuron_ref.index) };
                if !neuron.visited {
                    neuron.visited = true;
                    neuron.latch = input(neuron_ref.index);
                }
                neuron.latch
            }
            Layer::Hidden => {
                // SAFETY: References are never out of bounds
                let neuron: *mut Hidden<H> = unsafe { hiddens.get_unchecked_mut(neuron_ref.index) };
                // SAFETY: Safe because we never modify the list nor do we revisit a node
                unsafe {
                    if !(*neuron).visited {
                        (*neuron).visited = true;
                        (*neuron).latch = <Tangential as Aggregator>::aggregate(
                            (*neuron)
                                .dentrites
                                .iter()
                                .map(|d| d.synapse * Self::update(inputs, hiddens, d.axon, input)),
                        );
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

    impl std::hash::Hash for Signal {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            let flat = unsafe { std::mem::transmute::<f32, u32>(self.0) };
            flat.hash(state);
        }
    }

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

        pub fn as_f32(self) -> f32 {
            self.0
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
