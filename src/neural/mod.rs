mod neuron;
mod signal;

use neuron::{Dentrite, Hidden, Input, Neuron, Output, Ref, Sink, Source};
pub use signal::Amplifier as Synapse;
pub use signal::Signal as Stimuli;

pub enum Axon<Input: Copy + Eq, Output: Copy + Eq, const H: u8> {
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

impl<Input: Copy + Eq, Output: Copy + Eq, const H: u8> Axon<Input, Output, H> {
    pub fn direct(input: Input, output: Output, synapse: Synapse) -> Self {
        Self::Direct {
            input,
            output,
            synapse,
        }
    }

    pub fn into_hidden(input: Input, output: u8, synapse: Synapse) -> Self {
        Self::IntoHidden {
            input,
            output: output % H,
            synapse,
        }
    }

    pub fn inter_hidden(input: u8, output: u8, synapse: Synapse) -> Self {
        Self::InterHidden {
            input: input % H,
            output: output % H,
            synapse,
        }
    }

    pub fn from_hidden(input: u8, output: Output, synapse: Synapse) -> Self {
        Self::FromHidden {
            input: input % H,
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
    pub fn new<const H: u8>(axons: impl Iterator<Item = Axon<I, O, H>>) -> Self {
        let mut inputs: Vec<Input<I>> = vec![];
        let mut hiddens: Vec<Hidden> = vec![];
        let mut outputs: Vec<Output<O>> = vec![];

        for axon in axons {
            match axon {
                Axon::Direct {
                    input,
                    output,
                    synapse,
                } => {
                    Self::make_synapse(input, output, synapse, &mut inputs, &mut outputs);
                }
                Axon::IntoHidden {
                    input,
                    output,
                    synapse,
                } => {
                    Self::make_synapse(input, output, synapse, &mut inputs, &mut hiddens);
                }
                Axon::InterHidden {
                    input,
                    output,
                    synapse,
                } => {
                    let hiddens: *mut Vec<Hidden> = &mut hiddens;
                    Self::make_synapse(
                        input,
                        output,
                        synapse,
                        unsafe { hiddens.as_mut().unwrap() },
                        unsafe { hiddens.as_mut().unwrap() },
                    );
                }
                Axon::FromHidden {
                    input,
                    output,
                    synapse,
                } => {
                    Self::make_synapse(input, output, synapse, &mut hiddens, &mut outputs);
                }
            }
        }

        Self {
            inputs,
            hiddens,
            outputs,
        }
    }

    fn make_synapse<In: Copy + Eq, Out: Copy + Eq, NIn: Source<In>, NOut: Sink<Out>>(
        input: In,
        output: Out,
        synapse: Synapse,
        inputs: &mut Vec<NIn>,
        outputs: &mut Vec<NOut>,
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
            inputs.push(NIn::new(input));
            inputs.len() - 1
        };

        let dentrite = Dentrite {
            neuron: Ref {
                hidden: NIn::hidden(),
                index: input_index,
            },
            synapse,
        };

        if let Some(output) = outputs.iter_mut().find(|i| i.index() == output) {
            // TODO: Can we precalculate what these two synapses would do to an input signal?
            output.dentrites_mut().push(dentrite);
        } else {
            let mut output = NOut::new(output);
            output.dentrites_mut().push(dentrite);
            outputs.push(output);
        }
    }

    pub fn stimulus(&mut self, input: impl Copy + Fn(usize) -> Stimuli) -> Vec<(O, Stimuli)> {
        self.clear_visits();

        let mut output = Vec::with_capacity(self.outputs.len());

        for o in &self.outputs {
            let signal = signal::aggregate(o.dentrites().map(|d| {
                d.synapse * Self::update(&mut self.inputs, &mut self.hiddens, d.neuron, input)
            }));
            output.push((o.index(), Stimuli::cap(signal)));
        }

        output
    }

    fn clear_visits(&mut self) {
        self.inputs.iter_mut().for_each(Source::unvisit);
        self.hiddens.iter_mut().for_each(Source::unvisit);
    }

    fn update(
        inputs: &mut Vec<Input<I>>,
        hiddens: &mut Vec<Hidden>,
        neuron_ref: Ref,
        input: impl Copy + Fn(usize) -> Stimuli,
    ) -> f32 {
        if neuron_ref.hidden {
            // SAFETY: References are never out of bounds
            let neuron: *mut Hidden = unsafe { hiddens.get_unchecked_mut(neuron_ref.index) };
            // SAFETY: Safe because we never modify the list nor do we revisit a node
            unsafe {
                if (*neuron).visit() {
                    (*neuron).latch(signal::aggregate(
                        (*neuron)
                            .dentrites()
                            .map(|d| d.synapse * Self::update(inputs, hiddens, d.neuron, input)),
                    ));
                }
                (*neuron).latched()
            }
        } else {
            // SAFETY: References are never out of bounds
            let neuron = unsafe { inputs.get_unchecked_mut(neuron_ref.index) };
            if neuron.visit() {
                neuron.latch(input(neuron_ref.index).as_f32());
            }
            neuron.latched()
        }
    }
}
