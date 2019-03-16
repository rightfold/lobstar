use std::f64::consts::PI;
use support::synth::Synth;

#[derive(Clone, Copy)]
pub struct SineWave;

impl Synth for SineWave {
    fn synth(&self, time: f64, _channel: usize) -> f64 {
        f64::sin(2.0 * PI * time)
    }
}
