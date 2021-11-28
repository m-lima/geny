use super::Synapse;

pub trait Neuron<I: Copy + Eq> {
    fn new(index: I) -> Self;
    fn index(&self) -> I;
}

pub trait Source<I: Copy + Eq>: Neuron<I> {
    fn hidden() -> bool;
    fn visit(&mut self) -> bool;
    fn unvisit(&mut self);
    fn latched(&self) -> f32;
    fn latch(&mut self, latch: f32);
}

pub trait Sink<I: Copy + Eq>: Neuron<I> {
    fn dentrites(&self) -> std::slice::Iter<'_, Dentrite>;
    fn dentrites_mut(&mut self) -> &mut Vec<Dentrite>;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ref {
    pub hidden: bool,
    pub index: usize,
}

pub struct Dentrite {
    pub neuron: Ref,
    pub synapse: Synapse,
}

pub struct Input<I: Copy + Eq> {
    index: I,
    latch: f32,
    visited: bool,
}

impl<I: Copy + Eq> Neuron<I> for Input<I> {
    fn new(index: I) -> Self {
        Self {
            index,
            latch: 0.0,
            visited: false,
        }
    }

    #[inline]
    fn index(&self) -> I {
        self.index
    }
}

impl<I: Copy + Eq> Source<I> for Input<I> {
    #[inline]
    fn hidden() -> bool {
        false
    }

    #[inline]
    fn visit(&mut self) -> bool {
        if self.visited {
            false
        } else {
            self.visited = true;
            true
        }
    }

    #[inline]
    fn unvisit(&mut self) {
        self.visited = false;
    }

    #[inline]
    fn latched(&self) -> f32 {
        self.latch
    }

    #[inline]
    fn latch(&mut self, latch: f32) {
        self.latch = latch;
    }
}

pub struct Hidden {
    index: u8,
    dentrites: Vec<Dentrite>,
    latch: f32,
    visited: bool,
}

impl Neuron<u8> for Hidden {
    fn new(index: u8) -> Self {
        Self {
            index,
            dentrites: vec![],
            latch: 0.0,
            visited: false,
        }
    }

    #[inline]
    fn index(&self) -> u8 {
        self.index
    }
}

impl Source<u8> for Hidden {
    #[inline]
    fn hidden() -> bool {
        false
    }

    #[inline]
    fn visit(&mut self) -> bool {
        if self.visited {
            false
        } else {
            self.visited = true;
            true
        }
    }

    #[inline]
    fn unvisit(&mut self) {
        self.visited = false;
    }

    #[inline]
    fn latched(&self) -> f32 {
        self.latch
    }

    #[inline]
    fn latch(&mut self, latch: f32) {
        self.latch = latch;
    }
}

impl Sink<u8> for Hidden {
    #[inline]
    fn dentrites(&self) -> std::slice::Iter<'_, Dentrite> {
        self.dentrites.iter()
    }

    #[inline]
    fn dentrites_mut(&mut self) -> &mut Vec<Dentrite> {
        &mut self.dentrites
    }
}

pub struct Output<I: Copy + Eq> {
    index: I,
    dentrites: Vec<Dentrite>,
}

impl<I: Copy + Eq> Neuron<I> for Output<I> {
    fn new(index: I) -> Self {
        Self {
            index,
            dentrites: vec![],
        }
    }

    #[inline]
    fn index(&self) -> I {
        self.index
    }
}

impl<I: Copy + Eq> Sink<I> for Output<I> {
    #[inline]
    fn dentrites(&self) -> std::slice::Iter<'_, Dentrite> {
        self.dentrites.iter()
    }

    #[inline]
    fn dentrites_mut(&mut self) -> &mut Vec<Dentrite> {
        &mut self.dentrites
    }
}
