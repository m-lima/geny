mod body;
mod mind;

use body::Body;
use mind::Mind;

pub struct Boop {
    mind: Mind,
    body: Body,
}

impl Boop {
    pub fn new(synapses: u16, hidden_neurons: u8) -> Self {
        Self {
            mind: Mind::random(synapses, hidden_neurons),
            body: Body::new(),
        }
    }

    #[inline]
    pub fn signature(&self) -> u32 {
        self.mind.genome().signature()
    }

    #[inline]
    pub fn mind_mut(&mut self) -> &mut Mind {
        &mut self.mind
    }

    #[inline]
    pub fn mate(&self, mate: &Self, mutability: f32, hidden_neurons: u8) -> Self {
        let mut spawn = self.mind.genome().combine(mate.mind.genome());
        if rand::random::<f32>() < mutability {
            spawn.mutate(hidden_neurons);
        }
        Self {
            mind: Mind::from(spawn),
            body: Body::new(),
        }
    }
}

impl std::ops::Deref for Boop {
    type Target = Body;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl std::ops::DerefMut for Boop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}
