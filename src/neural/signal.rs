/// Float capped to the 0..1 range
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Signal(f32);

impl Signal {
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
        Self(f32::from(value) / f32::from(u8::MAX))
    }
}

/// Float capped to the -4..4 range
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Amplifier(f32);

impl Amplifier {
    pub fn new(value: f32) -> Self {
        Self(value.min(4.).max(-4.))
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
