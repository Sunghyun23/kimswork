use loop_club::{dsp::FxState, model::*};
#[test]
fn repeat_freezes_the_captured_segment_and_ignores_new_input() {
    let mut fx = FxState::new(8000);
    for i in 0..4000 {
        fx.process(
            Frame::mono((i % 100) as f32 / 200.),
            Fx::default(),
            8000.,
            240.,
        );
    }
    let p = Fx {
        effect: Effect::Repeat,
        a: 0.,
        b: 0.5,
        mix: 1.,
    };
    for _ in 0..6000 {
        fx.process(Frame::ZERO, p, 8000., 240.);
    }
    let mut peak = 0_f32;
    let mut max_diff = 0_f32;
    let mut last = [0_f32; 250];
    for (i, x) in last.iter_mut().enumerate() {
        *x = fx.process(Frame::ZERO, p, 8000., 240.).l;
        peak = peak.max(*x);
        assert!(i < 250)
    }
    for x in last {
        max_diff = max_diff.max((x - fx.process(Frame::ZERO, p, 8000., 240.).l).abs())
    }
    assert!(peak > 0.1);
    assert!(max_diff < 0.0001);
}
#[test]
fn tape_stop_does_not_resume_at_history_capacity() {
    let mut fx = FxState::new(8000);
    let p = Fx {
        effect: Effect::TapeStop,
        a: 0.,
        b: 0.5,
        mix: 1.,
    };
    let mut late = 0_f32;
    for i in 0..40000 {
        let s = fx.process(Frame::mono(0.3), p, 8000., 88.);
        if i > 30000 {
            late = late.max(s.peak());
        }
    }
    assert!(late < 0.0001);
}
#[test]
fn every_effect_is_finite_for_full_control_range() {
    for effect in [
        Effect::Off,
        Effect::Drive,
        Effect::Filter,
        Effect::Compressor,
        Effect::LoFi,
        Effect::Isolator,
        Effect::Repeat,
        Effect::Reverse,
        Effect::TapeStop,
        Effect::DubEcho,
        Effect::Crush,
        Effect::Wobble,
    ] {
        let mut fx = FxState::new(8000);
        for a in [0., 0.5, 1.] {
            for b in [0., 1.] {
                let p = Fx {
                    effect,
                    a,
                    b,
                    mix: 1.,
                };
                for i in 0..1500 {
                    let out = fx.process(Frame::mono((i as f32 * 0.1).sin() * 0.2), p, 8000., 40.);
                    assert!(out.finite());
                    assert!(out.peak() < 4.);
                }
            }
        }
    }
}
