extern crate lobstar;

use lobstar::support::synth::Synth;
use lobstar::support::synth::envelope;
use lobstar::support::synth::oscillate;
use lobstar::support::synth;
use std::io;
use std::mem;

fn encode(sample_value: f64) -> [u8; 8] {
    unsafe { mem::transmute(sample_value) }
}

fn play<W, S>(mut w: W, s: &S, t: f64) -> io::Result<()>
    where W: io::Write, S: Synth {
    let r = 44100.0;
    for n in (0 .. (t * r) as i64).map(|n| n as f64) {
        let v = s.synth(n / r, 0);
        w.write_all(&encode(v))?;
    }
    Ok(())
}

fn beat() -> impl Clone + Copy + Synth {
    let C4 = 261.6256;
    let D4 = 293.6648;
    let E4 = 329.6276;

    let tick = |f|
        oscillate::sine_wave()
            .tmul(f)
            .clip(-0.3, 0.3)
            .mul(envelope::adhsr(0.002, 0.005, 0.003, 0.7, 0.17))
    ;

    let tick1 = tick(C4).shift(0.0);
    let tick2 = tick(C4).shift(0.2);
    let tick3 = tick(C4).shift(0.4);
    let tick4 = tick(C4).shift(0.6);

    let tick5 = tick(E4).shift(0.8);
    let tick6 = tick(E4).shift(1.0);
    let tick7 = tick(E4).shift(1.2);
    let tick8 = tick(E4).shift(1.4);

    let tick9 = tick(D4).shift(1.6);
    let tickA = tick(D4).shift(1.8);
    let tickB = tick(D4).shift(2.0);
    let tickC = tick(D4).shift(2.2);

    let tickD = tick(E4).shift(2.4);
    let tickE = tick(E4).shift(2.6);
    let tickF = tick(E4).shift(2.8);
    let tick0 = tick(E4).shift(3.0);

    synth::constant(0.0)
    .add(tick1).add(tick2).add(tick3).add(tick4)
    .add(tick5).add(tick6).add(tick7).add(tick8)
    .add(tick9).add(tickA).add(tickB).add(tickC)
    .add(tickD).add(tickE).add(tickF).add(tick0)
}

fn main() -> io::Result<()> {
    let synth =
        synth::constant(0.0)
        .add(beat().shift(0.0))
        .add(beat().shift(3.2))
        .add(beat().shift(6.4))
        .add(beat().shift(9.6))
        .mul(synth::constant(0.1));

    let stdout = io::stdout();
    play(stdout.lock(), &synth, 12.8)?;
    Ok(())
}
