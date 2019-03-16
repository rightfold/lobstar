use support::synth::Synth;

#[derive(Clone, Copy)]
pub struct Constant {
    pub sample: f64,
}

impl Synth for Constant {
    fn synth(&self, _time: f64, _channel: usize) -> f64 {
        self.sample
    }
}
