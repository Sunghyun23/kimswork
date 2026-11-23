use loop_club::{
    dsp::FxState,
    engine::Engine,
    model::*,
    ui::{Snapshot, Ui},
};
fn params(effect: Effect) -> Fx {
    Fx {
        effect,
        a: 0.5,
        b: 1.,
        mix: 1.,
        modulation: FxMod {
            sync: true,
            division: 7,
            shape: 0,
        },
    }
}
#[test]
fn musical_rate_has_straight_dotted_triplet_and_follows_tempo() {
    let mut p = params(Effect::Tremolo);
    assert!((p.rate_hz(120.) - 4.).abs() < 1e-5);
    p.modulation.division = 8;
    assert!((p.rate_hz(120.) - 8. / 3.).abs() < 1e-5);
    assert_eq!(p.rate_label(), "1/8D");
    p.modulation.division = 9;
    assert_eq!(p.rate_label(), "1/8T");
    assert!((p.rate_hz(120.) - 6.).abs() < 1e-5);
    assert!((p.rate_hz(60.) - 3.).abs() < 1e-5);
    let old: Fx = serde_json::from_str(r#"{"effect":"Wobble","a":0.2,"b":0.1,"mix":0.3}"#).unwrap();
    assert!(!old.modulation.sync);
    assert_eq!(old.modulation.division, 7);
    assert!((old.rate_hz(120.) - 1.).abs() < 1e-5);
}
#[test]
fn tremolo_has_full_depth_and_pan_moves_audio_without_loudness_pumping() {
    let mut t = FxState::new(8000);
    let mut p = FxState::new(8000);
    let mut one = FxState::new(8000);
    let mut min = 1f32;
    let mut max = 0f32;
    let mut left = false;
    let mut right = false;
    let mut moved = 0f32;
    for i in 0..12000 {
        let x = t.process(Frame::mono(0.2), params(Effect::Tremolo), 8000., 120.);
        let y = p.process(Frame::mono(0.1), params(Effect::AutoPan), 8000., 120.);
        let z = one.process(
            Frame { l: 0.1, r: 0. },
            params(Effect::AutoPan),
            8000.,
            120.,
        );
        if i > 8000 {
            min = min.min(x.l);
            max = max.max(x.l);
            left |= y.l > y.r + 0.04;
            right |= y.r > y.l + 0.04;
            assert!((y.l * y.l + y.r * y.r - 0.02).abs() < 1e-4);
            moved = moved.max(z.r);
        }
    }
    assert!(min < 0.005 && max > 0.195, "{min} {max}");
    assert!(left && right && moved > 0.04);
}
#[test]
fn zero_depth_is_transparent_and_filter_sweep_changes_high_frequency_energy() {
    for effect in [Effect::Tremolo, Effect::AutoPan, Effect::AutoFilter] {
        let mut s = FxState::new(8000);
        let mut p = params(effect);
        p.b = 0.;
        for i in 0..8000 {
            let input = Frame {
                l: (i as f32 * 0.13).sin() * 0.2,
                r: (i as f32 * 0.17).sin() * 0.1,
            };
            let y = s.process(input, p, 8000., 120.);
            assert!((input.l - y.l).abs() < 1e-6 && (input.r - y.r).abs() < 1e-6);
        }
    }
    let mut s = FxState::new(8000);
    let mut energy = [0f32; 8];
    for i in 0..16000 {
        let input = Frame::mono((i as f32 * std::f32::consts::TAU * 3000. / 8000.).sin() * 0.2);
        let y = s.process(input, params(Effect::AutoFilter), 8000., 120.);
        assert!(y.finite());
        if i >= 8000 {
            energy[((i - 8000) / 250) % 8] += y.l * y.l;
        }
    }
    assert!(
        energy.iter().copied().fold(0., f32::max)
            > energy.iter().copied().fold(f32::MAX, f32::min) * 2.
    );
}
#[test]
fn settings_edit_sync_shape_without_turning_effect_on_and_save_new_fields() {
    let mut e = Engine::new(8000, 100., 8000);
    let mut u = Ui::default();
    u.page = Page::TrackFx;
    u.fx_focus = 11;
    u.open_fx_settings(&Snapshot::capture(&e));
    assert_eq!(u.page, Page::FxSettings);
    assert_eq!(u.edited_fx(&Snapshot::capture(&e)).effect, Effect::Tremolo);
    u.soft(2, false, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    u.knob(0, 7. / 15., &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    u.encoder(1, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    u.soft(1, false, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
    let p = e.tracks[0].fx[0];
    assert!(p.modulation.sync);
    assert_eq!(p.modulation.division, 8);
    assert_eq!(p.modulation.shape, 1);
    assert_eq!(Snapshot::capture(&e).fx(320).modulation.division, 8);
    let path = std::env::temp_dir().join(format!("lc-mod-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    let loaded = loop_club::project::load(&path, 8000, 8000).unwrap();
    assert_eq!(loaded.tracks[0].fx[0].modulation.division, 8);
    assert_eq!(loaded.tracks[0].fx[0].modulation.shape, 1);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn master_mod_pads_hold_latch_refuse_third_and_do_not_toggle_old_custom_tiles() {
    let mut e = Engine::new(8000, 100., 8000);
    let mut u = Ui::default();
    u.page = Page::Texture;
    for (pad, shift) in [(8, false), (9, true)] {
        u.pad(pad, true, 90, shift, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap()
        });
    }
    assert_eq!(e.performance[0].effect, Effect::Tremolo);
    assert_eq!(e.performance[1].effect, Effect::AutoPan);
    u.pad(10, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert_eq!(u.fx_refused, Some(10));
    u.pad(6, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert_eq!(e.performance[0].effect, Effect::Tremolo);
    u.page = Page::Loop;
    u.pad(8, false, 0, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert_eq!(e.performance[0].effect, Effect::Off);
    assert_eq!(e.performance[1].effect, Effect::AutoPan);
}
#[test]
fn sound_lfo_edit_keeps_legacy_volume_settings_and_edits_filter_depth() {
    let mut e = Engine::new(8000, 100., 8000);
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].synth.lfo_volume = 0.3;
    let mut u = Ui::default();
    u.page = Page::Play;
    u.synth_page = 6;
    u.shift_view = true;
    u.knob(2, 0.4, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(e.tracks[0].synth.lfo_volume, 0.3);
    assert!((e.tracks[0].synth.lfo_filter - 1.6).abs() < 1e-5);
}

#[test]
fn modulation_shapes_divisions_and_extremes_stay_bounded() {
    for effect in [Effect::Tremolo, Effect::AutoPan, Effect::AutoFilter] {
        for shape in 0..6 {
            for division in [0, 7, 15] {
                let mut s = FxState::new(8000);
                let mut p = params(effect);
                p.modulation.shape = shape;
                p.modulation.division = division;
                for i in 0..2000 {
                    let x = (i as f32 * 0.3).sin() * 0.2;
                    let y = s.process(Frame { l: x, r: -x * 0.5 }, p, 8000., 240.);
                    assert!(y.finite() && y.peak() < 0.5);
                }
            }
        }
    }
    let mut e = Engine::new(8000, 100., 8000);
    assert_eq!(loop_club::ui::synth_page_count(&Snapshot::capture(&e)), 7);
    e.tracks[0].synth.lfo_volume = 0.2;
    assert_eq!(loop_club::ui::synth_page_count(&Snapshot::capture(&e)), 8);
}

#[test]
fn fixed_master_slot_settings_stay_on_that_slot_and_board_rate_uses_sync_division() {
    let mut e = Engine::new(8000, 100., 8000);
    let mut u = Ui::default();
    u.page = Page::Texture;
    u.texture_slot = 7;
    u.open_fx_settings(&Snapshot::capture(&e));
    u.knob(1, 0.8, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(e.performance[1].b, 0.8);
    assert_eq!(e.performance[0].b, 0.5);
    u.page = Page::TrackFx;
    e.tracks[0].fx[0] = params(Effect::AutoPan);
    u.fx_slot = 0;
    u.pad(12, true, 80, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    u.knob(0, 9. / 15., &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert_eq!(e.tracks[0].fx[0].modulation.division, 9);
    assert_eq!(e.tracks[0].fx[0].a, 0.5);
}

#[test]
fn settings_back_keeps_the_edited_slot_selected() {
    let mut e = Engine::new(8000, 100., 8000);
    e.tracks[0].fx[0] = params(Effect::Tremolo);
    e.tracks[0].fx[1] = params(Effect::AutoPan);
    let mut u = Ui::default();
    u.page = Page::TrackFx;
    u.fx_focus = 12;
    u.open_fx_settings(&Snapshot::capture(&e));
    assert_eq!(u.fx_slot, 1);
    u.soft(0, false, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(u.fx_slot, 1);
    u.knob(1, 0.2, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(e.tracks[0].fx[1].b, 0.2);
    assert_eq!(e.tracks[0].fx[0].b, 1.);
}

#[test]
fn saving_a_held_master_mod_bypasses_it_on_recall_and_retains_latched_mod() {
    let mut e = Engine::new(8000, 100., 8000);
    for (slot, effect, temporary) in [(0, Effect::AutoPan, true), (1, Effect::Tremolo, false)] {
        e.command(loop_club::engine::Command::FxSelect {
            target: 1,
            track: 0,
            slot,
            effect,
            temporary,
        })
        .unwrap();
    }
    let path = std::env::temp_dir().join(format!("lc-held-mod-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    assert!(!e.texture.master_bypass[0]);
    let loaded = loop_club::project::load(&path, 8000, 8000).unwrap();
    assert!(loaded.texture.master_bypass[0]);
    assert!(!loaded.texture.master_bypass[1]);
    assert_eq!(loaded.performance[1].effect, Effect::Tremolo);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn temporary_texture_and_duck_are_not_saved_and_latched_texture_is_retained() {
    let mut e = Engine::new(8000, 100., 8000);
    for (slot, held) in [(0, true), (1, false)] {
        for (index, value) in [(7, f32::from(held)), (3, 1.)] {
            e.command(loop_club::engine::Command::TextureValue { slot, index, value })
                .unwrap();
        }
    }
    e.sidechain.depth = 0.7;
    e.sidechain.enabled = true;
    e.command(loop_club::engine::Command::SidechainHeld(true))
        .unwrap();
    let path = std::env::temp_dir().join(format!("lc-fixed-held-{}.lcproj", std::process::id()));
    loop_club::project::save(&e, &path).unwrap();
    let loaded = loop_club::project::load(&path, 8000, 8000).unwrap();
    assert!(!loaded.texture.enabled[0]);
    assert!(loaded.texture.enabled[1]);
    assert!(!loaded.sidechain.enabled);
    assert_eq!(loaded.sidechain.depth, 0.7);
    std::fs::remove_file(path).unwrap();
    e.release_temporary_fx();
    assert!(!e.texture.enabled[0] && e.texture.enabled[1] && !e.sidechain.enabled);
}

#[test]
fn settings_fader_controls_the_original_track_or_master() {
    for master in [false, true] {
        let mut e = Engine::new(8000, 100., 8000);
        let mut u = Ui::default();
        u.navigate(
            if master { Page::Texture } else { Page::TrackFx },
            &Snapshot::capture(&e),
        );
        if master {
            u.texture_slot = 8;
        } else {
            u.fx_focus = 11;
        }
        u.open_fx_settings(&Snapshot::capture(&e));
        u.fader(1., &Snapshot::capture(&e), |c| e.command(c).unwrap());
        u.fader(0.4, &Snapshot::capture(&e), |c| e.command(c).unwrap());
        if master {
            assert_eq!(e.master, 0.4);
            assert_eq!(e.tracks[0].mix.level, 0.7);
        } else {
            assert_eq!(e.tracks[0].mix.level, 0.4);
            assert_eq!(e.master, 0.8);
        }
    }
}
