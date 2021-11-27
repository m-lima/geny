pub use signal::Signal;
use signal::{Aggregator, Amplifier, Tangential};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Layer {
    Input,
    Internal,
    Output,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pointer {
    layer: Layer,
    neuron: usize,
}

struct Dentrite {
    axon: Pointer,
    synapse: Amplifier,
}

pub trait Input<State> {
    fn update(&self, state: &State) -> Signal;
}

struct InputNeuron<State, In: Input<State>> {
    input: In,
    latch: Signal,
    visited: bool,
    _marker: std::marker::PhantomData<State>,
}

impl<State, In: Input<State>> InputNeuron<State, In> {
    fn new(input: In) -> Self {
        Self {
            input,
            latch: Signal::new(),
            visited: false,
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait Output<State> {
    fn update(&self, state: &mut State);
}

struct OutputNeuron<State, Out: Output<State>> {
    output: Out,
    dentrites: Vec<Dentrite>,
    _marker: std::marker::PhantomData<State>,
}

impl<State, Out: Output<State>> OutputNeuron<State, Out> {
    fn new(output: Out) -> Self {
        Self {
            output,
            dentrites: vec![],
            _marker: std::marker::PhantomData,
        }
    }
}

struct InternalNeuron {
    dentrites: Vec<Dentrite>,
    latch: Signal,
    visited: bool,
}

impl InternalNeuron {
    fn new() -> Self {
        Self {
            dentrites: vec![],
            latch: Signal::new(),
            visited: false,
        }
    }
}

pub struct Brain<
    State,
    In: Input<State>,
    Out: Output<State>,
    const INPUTS: usize,
    const INTERNALS: usize,
    const OUTPUTS: usize,
> {
    inputs: [InputNeuron<State, In>; INPUTS],
    internals: [InternalNeuron; INTERNALS],
    outputs: [OutputNeuron<State, Out>; OUTPUTS],
}

impl<
        State,
        In: Input<State>,
        Out: Output<State>,
        const INPUTS: usize,
        const INTERNALS: usize,
        const OUTPUTS: usize,
    > Brain<State, In, Out, INPUTS, INTERNALS, OUTPUTS>
{
    pub fn new(inputs: [In; INPUTS], outputs: [Out; OUTPUTS]) -> Self {
        Self {
            inputs: inputs.map(InputNeuron::new),
            internals: [0; INTERNALS].map(|_| InternalNeuron::new()),
            outputs: outputs.map(OutputNeuron::new),
        }
    }

    fn update(&mut self, pointer: Pointer, state: &State) -> Signal {
        match pointer.layer {
            Layer::Input => {
                let neuron = &mut self.inputs[pointer.neuron];
                if !neuron.visited {
                    neuron.visited = true;
                    neuron.latch = neuron.input.update(state);
                }
                neuron.latch
            }
            Layer::Internal => {
                let neuron: *mut InternalNeuron = &mut self.internals[pointer.neuron];
                // TODO: Avoid this unsafe. Maybe reloading the index at each access
                unsafe {
                    if !(*neuron).visited {
                        (*neuron).visited = true;
                        (*neuron).latch = <Tangential as Aggregator>::aggregate(
                            (*neuron)
                                .dentrites
                                .iter()
                                .map(|d| d.synapse * self.update(d.axon, state)),
                        );
                    }
                    (*neuron).latch
                }
            }
            Layer::Output => unreachable!("Output layer can never be a source of signal"),
        }
    }

    pub fn step(&mut self, state: &mut State) {
        self.inputs.iter_mut().for_each(|i| i.visited = false);
        self.internals.iter_mut().for_each(|i| i.visited = false);

        for i in 0..self.outputs.len() {
            let neuron: *const OutputNeuron<State, Out> = &self.outputs[i];
            unsafe {
                let probability = <Tangential as Aggregator>::aggregate(
                    (*neuron)
                        .dentrites
                        .iter()
                        .map(|d| d.synapse * self.update(d.axon, state)),
                );
                if rand::random::<f32>() < probability.as_f32() {
                    (*neuron).output.update(state);
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
