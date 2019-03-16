use super::Synth;

#[derive(Clone, Copy)]
pub struct Mul<S, T> {
    pub inner1: S,
    pub inner2: T,
}

impl<S, T> Synth for Mul<S, T> where S: Synth, T: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        self.inner1.synth(time, channel) *
        self.inner2.synth(time, channel)
    }
}
