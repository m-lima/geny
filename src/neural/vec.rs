pub use signal::Amplifier as Synapse;
pub use signal::Signal as Stimuli;

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
    synapse: Synapse,
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
    latch: f32,
    visited: bool,
}

impl<I: Copy + Eq> Input<I> {
    fn new(index: I) -> Self {
        Self {
            index,
            latch: 0.0,
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

struct Hidden {
    index: u8,
    dentrites: Vec<Dentrite>,
    latch: f32,
    visited: bool,
}

impl Hidden {
    fn new(index: u8) -> Self {
        Self {
            index,
            dentrites: vec![],
            latch: 0.0,
            visited: false,
        }
    }
}

impl Neuron<u8> for Hidden {
    #[inline]
    fn index(&self) -> u8 {
        self.index
    }
}

impl OutNeuron<u8> for Hidden {
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

pub enum Gene<Input: Copy + Eq, Output: Copy + Eq, const H: u8> {
    Direct {
        input: Input,
        output: Output,
        synapse: Synapse,
    },
    IntoHidden {
        input: Input,
        output: u8,
        synapse: Synapse,
    },
    InterHidden {
        input: u8,
        output: u8,
        synapse: Synapse,
    },
    FromHidden {
        input: u8,
        output: Output,
        synapse: Synapse,
    },
}

impl<Input: Copy + Eq, Output: Copy + Eq, const H: u8> Gene<Input, Output, H> {
    pub fn direct(input: Input, output: Output, synapse: Synapse) -> Self {
        Self::Direct {
            input,
            output,
            synapse,
        }
    }
}

pub struct Brain<I: Copy + Eq, O: Copy + Eq> {
    inputs: Vec<Input<I>>,
    hiddens: Vec<Hidden>,
    outputs: Vec<Output<O>>,
}

impl<I: Copy + Eq, O: Copy + Eq> Brain<I, O> {
    pub fn new<const H: u8>(genome: &[Gene<I, O, H>]) -> Self {
        const AMPLIFIER: f32 = 4.0;

        let mut inputs: Vec<Input<I>> = vec![];
        let mut hiddens: Vec<Hidden> = vec![];
        let mut outputs: Vec<Output<O>> = vec![];

        for gene in genome {
            match gene {
                Gene::Direct {
                    input,
                    output,
                    synapse,
                } => {
                    Self::make_synapse(
                        *input,
                        *output,
                        *synapse,
                        &mut inputs,
                        &mut outputs,
                        Input::new,
                        Output::new,
                    );
                }
                Gene::IntoHidden {
                    input,
                    output,
                    synapse,
                } => {
                    Self::make_synapse(
                        *input,
                        *output,
                        *synapse,
                        &mut inputs,
                        &mut hiddens,
                        Input::new,
                        Hidden::new,
                    );
                }
                Gene::InterHidden {
                    input,
                    output,
                    synapse,
                } => {
                    let hiddens: *mut Vec<Hidden> = &mut hiddens;
                    Self::make_synapse(
                        *input,
                        *output,
                        *synapse,
                        unsafe { hiddens.as_mut().unwrap() },
                        unsafe { hiddens.as_mut().unwrap() },
                        Hidden::new,
                        Hidden::new,
                    );
                }
                Gene::FromHidden {
                    input,
                    output,
                    synapse,
                } => {
                    Self::make_synapse(
                        *input,
                        *output,
                        *synapse,
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
        synapse: Synapse,
        inputs: &mut Vec<NIn>,
        outputs: &mut Vec<NOut>,
        in_builder: BIn,
        out_builder: BOut,
    ) {
        // ALLOWED: We need to mutate `inputs` in the else case
        #[allow(clippy::option_if_let_else)]
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
            synapse,
        };

        if let Some(output) = outputs.iter_mut().find(|i| i.index() == output) {
            // TODO: Can we precalculate what these two synapses would do to an input signal?
            output.dentrites_mut().push(dentrite);
        } else {
            let mut output = out_builder(output);
            output.dentrites_mut().push(dentrite);
            outputs.push(output);
        }
    }

    pub fn stimulus(&mut self, input: impl Copy + Fn(usize) -> Stimuli) -> Vec<(O, Stimuli)> {
        self.clear_visits();

        let mut output = Vec::with_capacity(self.outputs.len());

        for o in &self.outputs {
            let signal = signal::aggregate(o.dentrites.iter().map(|d| {
                d.synapse * Self::update(&mut self.inputs, &mut self.hiddens, d.axon, input)
            }));
            output.push((o.index, Stimuli::cap(signal)));
        }

        output
    }

    fn clear_visits(&mut self) {
        self.inputs.iter_mut().for_each(|i| i.visited = false);
        self.hiddens.iter_mut().for_each(|i| i.visited = false);
    }

    fn update(
        inputs: &mut Vec<Input<I>>,
        hiddens: &mut Vec<Hidden>,
        neuron_ref: NeuronRef,
        input: impl Copy + Fn(usize) -> Stimuli,
    ) -> f32 {
        match neuron_ref.layer {
            Layer::Input => {
                // SAFETY: References are never out of bounds
                let neuron = unsafe { inputs.get_unchecked_mut(neuron_ref.index) };
                if !neuron.visited {
                    neuron.visited = true;
                    neuron.latch = input(neuron_ref.index).as_f32();
                }
                neuron.latch
            }
            Layer::Hidden => {
                // SAFETY: References are never out of bounds
                let neuron: *mut Hidden = unsafe { hiddens.get_unchecked_mut(neuron_ref.index) };
                // SAFETY: Safe because we never modify the list nor do we revisit a node
                unsafe {
                    if !(*neuron).visited {
                        (*neuron).visited = true;
                        (*neuron).latch = signal::aggregate(
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
    /// Float capped to the 0..1 range
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub struct Signal(f32);

    impl Signal {
        pub fn half() -> Self {
            Self(0.5)
        }

        pub fn cap(value: f32) -> Self {
            Self(value.min(1.).max(0.))
        }

        #[inline]
        pub fn as_f32(self) -> f32 {
            self.0
        }
    }

    impl From<bool> for Signal {
        fn from(value: bool) -> Self {
            if value {
                Self(1.)
            } else {
                Self(0.)
            }
        }
    }

    impl From<u8> for Signal {
        fn from(value: u8) -> Self {
            Self(value as f32 / (u8::MAX as f32))
        }
    }

    /// Float capped to the -4..4 range
    #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
    pub struct Amplifier(f32);

    impl Amplifier {
        pub fn new(value: f32) -> Self {
            Self(value.min(4.).max(-4.))
        }

        #[inline]
        pub fn as_f32(self) -> f32 {
            self.0
        }
    }

    impl std::ops::Mul<f32> for Amplifier {
        type Output = f32;

        #[inline]
        fn mul(self, rhs: f32) -> Self::Output {
            self.0 * rhs
        }
    }

    pub fn aggregate(inputs: impl Iterator<Item = f32>) -> f32 {
        inputs.fold(0.0, |a, c| a + c).tanh()
    }
}
