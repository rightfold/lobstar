//! Envelope synths.

use support::synth::Synth;

mod adhsr;

/// Synth an ADHSR envelope with a peak amplitude of 1.
///
/// Since the returned synth does not produce waves, you should multiply it
/// with one that does.
pub fn adhsr(attack: f64, decay: f64, hold: f64, sustain: f64,
             release: f64) -> impl Clone + Copy + Synth {
    adhsr::Adhsr{attack, decay, hold, sustain, release}
}

/// Synth an H envelope with an amplitude of 1.
///
/// Since the returned synth does not produce waves, you should multiply it
/// with one that does.
pub fn h(hold: f64) -> impl Clone + Copy + Synth {
    adhsr(0.0, 0.0, hold, 1.0, 0.0)
}
