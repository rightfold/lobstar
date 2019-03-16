use super::Synth;

#[derive(Clone, Copy)]
pub struct Tmul<S> {
    pub inner: S,
    pub factor: f64,
}

impl<S> Synth for Tmul<S> where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        self.inner.synth(time * self.factor, channel)
    }
}
