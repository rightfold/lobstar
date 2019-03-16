//! Oscillating synths.

use support::synth::Synth;

mod sine_wave;

/// Synth a sine wave with an amplitude of 1 and a frequency of 1 Hz.
pub fn sine_wave() -> impl Clone + Copy + Synth {
    sine_wave::SineWave
}
