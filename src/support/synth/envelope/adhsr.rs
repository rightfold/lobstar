use support::synth::Synth;

#[derive(Clone, Copy)]
pub struct Adhsr {
    pub attack:  f64,
    pub decay:   f64,
    pub hold:    f64,
    pub sustain: f64,
    pub release: f64,
}

impl Synth for Adhsr {
    #[allow(non_snake_case, unused_assignments)]
    fn synth(&self, time: f64, _channel: usize) -> f64 {
        let mut t = time;
        let A = self.attack;
        let D = self.decay;
        let H = self.hold;
        let S = self.sustain;
        let R = self.release;

        if t < 0.0 { return 0.0 }
        t -= 0.0;

        if t < A { return t / A }
        t -= A;

        if t < D { return S + (1.0 - S) * (1.0 - t / D) }
        t -= D;

        if t < H { return S }
        t -= H;

        if t < R { return S * (1.0 - t / R) }
        t -= R;

        return 0.0;
    }
}
