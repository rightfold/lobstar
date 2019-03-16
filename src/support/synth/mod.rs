//! Synths.

use std::rc::Rc;

mod add; use self::add::*;
mod clip; use self::clip::*;
mod mul; use self::mul::*;
mod shift; use self::shift::*;
mod tmul; use self::tmul::*;

mod constant;

pub mod envelope;
pub mod oscillate;

/// A synth produces a sample at each point in time and for each channel.
pub trait Synth {
    /// Produce a sample.
    fn synth(&self, time: f64, channel: usize) -> f64;

    /// Add the corresponding samples of two synths.
    fn add<S>(self, other: S) -> Add<Self, S> where Self: Sized, S: Synth {
        Add{inner1: self, inner2: other}
    }

    /// Multiply the corresponding samples of two synths.
    fn mul<S>(self, other: S) -> Mul<Self, S> where Self: Sized, S: Synth {
        Mul{inner1: self, inner2: other}
    }

    /// Shift the samples of a synth.
    fn shift(self, delay: f64) -> Shift<Self> where Self: Sized {
        Shift{inner: self, delay}
    }

    /// Multiply the frequency of a synth.
    fn tmul(self, factor: f64) -> Tmul<Self> where Self: Sized {
        Tmul{inner: self, factor}
    }

    fn clip(self, lower: f64, upper: f64) -> Clip<Self> where Self: Sized {
        Clip{inner: self, lower, upper}
    }
}

/// Synth that produces the same sample regardless of time or channel.
///
/// Since the returned synth does not produce waves, you should multiply it
/// with one that does.
pub fn constant(sample: f64) -> impl Clone + Copy + Synth {
    constant::Constant{sample}
}

impl<'a, S> Synth for &'a S where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        (**self).synth(time, channel)
    }
}

impl<'a, S> Synth for &'a mut S where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        (**self).synth(time, channel)
    }
}

impl<'a, S> Synth for Box<S> where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        (**self).synth(time, channel)
    }
}

impl<'a, S> Synth for Rc<S> where S: Synth {
    fn synth(&self, time: f64, channel: usize) -> f64 {
        (**self).synth(time, channel)
    }
}
