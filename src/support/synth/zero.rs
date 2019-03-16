use super::Synth;

#[derive(Clone, Copy)]
pub struct Zero;

impl Synth for Zero {
    fn synth(&self, _time: f64, _channel: usize) -> f64 {
        0.0
    }
}
