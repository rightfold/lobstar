use super::Synth;

#[derive(Clone, Copy)]
pub struct Clip<S> {
    pub inner: S,
    pub lower: f64,
    pub upper: f64,
}

impl<S> Synth for Clip<S> where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        self.inner.synth(time, channel).max(self.lower).min(self.upper)
    }
}

