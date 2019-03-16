use super::Synth;

#[derive(Clone, Copy)]
pub struct Shift<S> {
    pub inner: S,
    pub delay: f64,
}

impl<S> Synth for Shift<S> where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        self.inner.synth(time - self.delay, channel)
    }
}
