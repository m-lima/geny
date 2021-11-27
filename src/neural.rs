use signal::{Aggregator, Amplifier, Signal};

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

struct Dentrite<const A: u8> {
    axon: Pointer,
    synapse: Amplifier<A>,
}

trait Input<State> {
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

trait Output<State> {
    fn update(&self, state: &mut State);
}

struct OutputNeuron<State, Out: Output<State>, const A: u8> {
    output: Out,
    dentrites: Vec<Dentrite<A>>,
    aggregator: Aggregator,
    _marker: std::marker::PhantomData<State>,
}

impl<State, Out: Output<State>, const A: u8> OutputNeuron<State, Out, A> {
    fn new(output: Out, aggregator: Aggregator) -> Self {
        Self {
            output,
            dentrites: vec![],
            aggregator,
            _marker: std::marker::PhantomData,
        }
    }
}

struct InternalNeuron<const A: u8> {
    dentrites: Vec<Dentrite<A>>,
    aggregator: Aggregator,
    latch: Signal,
    visited: bool,
}

impl<const A: u8> InternalNeuron<A> {
    fn new(aggregator: Aggregator) -> Self {
        Self {
            dentrites: vec![],
            aggregator,
            latch: Signal::new(),
            visited: false,
        }
    }
}

struct Brain<
    State,
    In: Input<State>,
    Out: Output<State>,
    const A: u8,
    const INPUTS: usize,
    const INTERNALS: usize,
    const OUTPUTS: usize,
> {
    inputs: [InputNeuron<State, In>; INPUTS],
    internals: [InternalNeuron<A>; INTERNALS],
    outputs: [OutputNeuron<State, Out, A>; OUTPUTS],
}

impl<
        State,
        In: Input<State>,
        Out: Output<State>,
        const A: u8,
        const INPUTS: usize,
        const INTERNALS: usize,
        const OUTPUTS: usize,
    > Brain<State, In, Out, A, INPUTS, INTERNALS, OUTPUTS>
{
    pub fn new(inputs: [In; INPUTS], outputs: [Out; OUTPUTS]) -> Self {
        Self {
            inputs: inputs.map(InputNeuron::new),
            internals: [0; INTERNALS].map(|_| InternalNeuron::new(Aggregator::Tangential)),
            outputs: outputs.map(|output| OutputNeuron::new(output, Aggregator::Tangential)),
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
                let neuron: *mut InternalNeuron<A> = &mut self.internals[pointer.neuron];
                unsafe {
                    if !(*neuron).visited {
                        (*neuron).visited = true;
                        (*neuron).latch = (*neuron).aggregator.aggregate(
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
            let neuron: *const OutputNeuron<State, Out, A> = &self.outputs[i];
            unsafe {
                let probability = (*neuron).aggregator.aggregate(
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

    #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
    pub struct Amplifier<const AMPLITUDE: u8>(f32);

    impl<const AMPLITUDE: u8> std::ops::Mul<Signal> for Amplifier<AMPLITUDE> {
        type Output = Signal;

        fn mul(self, rhs: Signal) -> Self::Output {
            Signal::cap(rhs.as_f32() * f32::from(AMPLITUDE))
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Aggregator {
        Linear,
        Tangential,
    }

    impl Aggregator {
        pub fn aggregate(self, inputs: impl Iterator<Item = Signal>) -> Signal {
            match self {
                Self::Linear => {
                    let sum = inputs
                        .enumerate()
                        .fold((0, 0.0), |a, c| (c.0, a.1 + c.1.as_f32()));
                    // ALLOWED: Because there cannot be that many synapses (not enough neurons to
                    // saturate 23 bits)
                    #[allow(clippy::cast_precision_loss)]
                    Signal::cap(sum.1 / sum.0 as f32)
                }
                Self::Tangential => Signal::cap(inputs.fold(0.0, |a, c| a + c.as_f32()).tanh()),
            }
        }
    }
}
