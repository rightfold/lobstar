use super::Synth;

#[derive(Clone, Copy)]
pub struct One;

impl Synth for One {
    fn synth(&self, _time: f64, _channel: usize) -> f64 {
        1.0
    }
}
