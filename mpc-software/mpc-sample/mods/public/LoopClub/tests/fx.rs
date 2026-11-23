use loop_club::{dsp::FxState, engine::Engine, model::*};
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
        ..Fx::default()
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
        ..Fx::default()
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
        Effect::Tremolo,
        Effect::AutoPan,
        Effect::AutoFilter,
    ] {
        let mut fx = FxState::new(8000);
        for a in [0., 0.5, 1.] {
            for b in [0., 1.] {
                let p = Fx {
                    effect,
                    a,
                    b,
                    mix: 1.,
                    ..Fx::default()
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

#[test]
fn repeat_turned_on_in_silence_captures_later_audio_instead_of_freezing_zero() {
    let mut state = FxState::new(8000);
    let repeat = Fx {
        effect: Effect::Repeat,
        a: 0.,
        b: 0.1,
        mix: 1.,
        ..Fx::default()
    };
    for _ in 0..4000 {
        state.process(Frame::ZERO, Fx::default(), 8000., 240.);
    }
    for _ in 0..2000 {
        state.process(Frame::ZERO, repeat, 8000., 240.);
    }
    let mut late = 0_f32;
    for n in 0..4000 {
        let x = state.process(Frame::mono(0.25), repeat, 8000., 240.);
        if n > 2000 {
            late = late.max(x.peak());
        }
    }
    assert!(
        late > 0.1,
        "Repeat trapped the silent buffer; late peak {late}"
    );
}

#[test]
fn repeat_after_old_audio_and_long_silence_waits_for_new_audio() {
    let mut state = FxState::new(8000);
    for _ in 0..2000 {
        state.process(Frame::mono(0.3), Fx::default(), 8000., 240.);
    }
    for _ in 0..4000 {
        state.process(Frame::ZERO, Fx::default(), 8000., 240.);
    }
    let repeat = Fx {
        effect: Effect::Repeat,
        a: 1.,
        b: 0.,
        mix: 1.,
        ..Fx::default()
    };
    for _ in 0..2000 {
        state.process(Frame::ZERO, repeat, 8000., 240.);
    }
    let mut peak = 0_f32;
    for i in 0..12000 {
        let s = state.process(Frame::mono(0.15), repeat, 8000., 240.);
        if i > 8000 {
            peak = peak.max(s.peak());
        }
    }
    assert!(peak > 0.1);
}

#[test]
fn repeat_reenabled_after_bypass_and_silence_rearms_for_new_audio() {
    let mut state = FxState::new(8000);
    let repeat = Fx {
        effect: Effect::Repeat,
        a: 0.,
        b: 0.,
        mix: 1.,
        ..Fx::default()
    };
    for _ in 0..2000 {
        state.process(Frame::mono(0.2), Fx::default(), 8000., 240.);
    }
    for _ in 0..1000 {
        state.process(Frame::mono(0.2), repeat, 8000., 240.);
    }
    for _ in 0..6000 {
        state.process(Frame::ZERO, Fx::default(), 8000., 240.);
    }
    for _ in 0..1000 {
        state.process(Frame::ZERO, repeat, 8000., 240.);
    }
    let mut late = 0_f32;
    for i in 0..4000 {
        let s = state.process(Frame::mono(0.25), repeat, 8000., 240.);
        if i > 2000 {
            late = late.max(s.peak());
        }
    }
    assert!(
        late > 0.1,
        "Repeat bypass/re-enable froze silence; late peak {late}"
    );
}

#[test]
fn pressed_effect_gets_inline_knob_routing_and_keeps_values_when_released() {
    use loop_club::ui::{Snapshot, Ui};
    let mut e = Engine::new(8000, 100., 16000);
    let mut ui = Ui::default();
    ui.page = Page::TrackFx;
    ui.pad(0, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    ui.knob(0, 0.23, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    ui.knob(2, 0.72, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    ui.pad(0, false, 0, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    let (_, slot, fx) = ui.quick_fx(&Snapshot::capture(&e)).unwrap();
    assert_eq!(slot, 0);
    assert_eq!(fx.effect, Effect::Filter);
    assert_eq!(fx.a, 0.23);
    assert_eq!(fx.mix, 0.72);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
    ui.knob(1, 0.36, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].fx[0].b, 0.36);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
}
#[test]
fn latest_pressed_pad_edits_only_its_slot_and_full_slots_are_protected() {
    use loop_club::ui::{Snapshot, Ui};
    let mut e = Engine::new(8000, 100., 16000);
    let mut ui = Ui::default();
    ui.page = Page::MasterFx;
    for pad in [0, 2] {
        ui.pad(pad, true, 80, false, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
    }
    ui.knob(0, 0.77, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.performance[0].a, 0.5);
    assert_eq!(e.performance[1].a, 0.77);
    ui.pad(3, true, 80, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(ui.quick_fx(&Snapshot::capture(&e)).is_none());
    ui.knob(0, 0.12, &Snapshot::capture(&e), |_| {
        panic!("must not edit occupied unrelated slot")
    });
    assert_eq!(e.performance[1].a, 0.77);
}
#[test]
fn adjusting_released_duck_depth_does_not_enable_it() {
    use loop_club::ui::{Snapshot, Ui};
    let mut e = Engine::new(8000, 100., 16000);
    let mut ui = Ui::default();
    ui.page = Page::Texture;
    ui.pad(5, true, 80, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    ui.pad(5, false, 0, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert!(!e.sidechain.enabled);
    ui.knob(0, 0.41, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.sidechain.depth, 0.41);
    assert!(!e.sidechain.enabled);
}
