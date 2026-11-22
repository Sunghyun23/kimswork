use loop_club::{
    engine::{Command, Engine},
    model::*,
    ui::{Snapshot, Ui},
};
fn state() -> (Engine, Ui) {
    (Engine::new(8000, 240., 8000), Ui::default())
}
fn press(ui: &mut Ui, e: &mut Engine, pad: usize, on: bool) {
    let s = Snapshot::capture(e);
    ui.pad(pad, on, 90, false, false, &s, |c| {
        e.command(c).unwrap();
    });
}
#[test]
fn fx_release_follows_original_track_after_navigation() {
    let (mut e, mut ui) = state();
    ui.page = Page::TrackFx;
    ui.latch = false; // Explicit optional momentary mode; default is now toggle.
    press(&mut ui, &mut e, 0, true);
    ui.selected = 1;
    e.command(Command::Select(1)).unwrap();
    ui.page = Page::Mix;
    e.tracks[1].fx[0].effect = Effect::Drive;
    press(&mut ui, &mut e, 0, false);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
    assert_eq!(e.tracks[1].fx[0].effect, Effect::Drive);
}
#[test]
fn releasing_superseded_fx_pad_does_not_disable_current_effect() {
    let (mut e, mut ui) = state();
    ui.page = Page::TrackFx;
    ui.latch = false; // Explicit optional momentary mode; default is now toggle.
    press(&mut ui, &mut e, 0, true);
    press(&mut ui, &mut e, 1, true);
    press(&mut ui, &mut e, 0, false);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Isolator);
    press(&mut ui, &mut e, 1, false);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
}
#[test]
fn fx_navigation_preserves_playback_and_parameters() {
    let (mut e, mut ui) = state();
    e.running = true;
    e.tracks[0].length_ticks = PPQ * 4;
    e.tracks[0].playing = true;
    e.tracks[0].fx[0].effect = Effect::Filter;
    ui.page = Page::Mix;
    for (key, page) in [(1, Page::TrackFx), (2, Page::Texture), (0, Page::Mix)] {
        ui.soft(key, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(ui.page, page);
        assert!(e.running && e.tracks[0].playing);
        assert_eq!(e.tracks[0].fx[0].effect, Effect::Filter);
    }
}
#[test]
fn synth_edit_preserves_octave() {
    let (mut e, mut ui) = state();
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].synth.octave = -1;
    ui.page = Page::Play;
    ui.knob(0, 0.4, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap();
    });
    assert_eq!(e.tracks[0].synth.octave, -1);
}
#[test]
fn insufficient_record_capacity_does_not_start_transport() {
    let mut e = Engine::new(8000, 240., 100);
    assert_eq!(e.command(Command::Record(0)), Err(Error::Capacity));
    assert!(!e.running);
    assert!(e.recorder.is_none());
}
#[test]
fn stop_clears_unfinished_audio_metadata() {
    let (mut e, _) = state();
    e.preroll = 0;
    e.command(Command::Record(0)).unwrap();
    for _ in 0..100 {
        e.process(Frame::mono(0.1));
    }
    assert_eq!(e.tracks[0].audio_frames, 100);
    e.command(Command::Stop).unwrap();
    assert_eq!(e.tracks[0].audio_frames, 0);
    assert!(e.tracks[0].empty());
}
#[test]
fn master_bus_fx_targets_do_not_change_track_or_other_bus() {
    let (mut e, mut ui) = state();
    ui.page = Page::MasterFx;
    press(&mut ui, &mut e, 13, true);
    press(&mut ui, &mut e, 0, true);
    assert_eq!(e.bus_fx[0].effect, Effect::Filter);
    assert_eq!(e.bus_fx[1].effect, Effect::Off);
    assert_eq!(e.performance[0].effect, Effect::Off);
    ui.latch = true;
    press(&mut ui, &mut e, 11, true);
    assert_eq!(e.bus_fx[0].effect, Effect::Off);
    assert_eq!(e.tracks[0].fx[0].effect, Effect::Off);
}
#[test]
fn cc_normalization_follows_sdk_float_geometry() {
    assert_eq!(loop_club::device::normalize_cc(0, 63.5, 127.), 0.5);
    assert_eq!(loop_club::device::normalize_cc(127, 0., 0.), 1.);
}
#[test]
fn fx_pages_remember_their_own_slot_and_latch() {
    let (e, mut ui) = state();
    let s = Snapshot::capture(&e);
    ui.navigate(Page::TrackFx, &s);
    ui.fx_slot = 1;
    ui.latch = true;
    ui.navigate(Page::MasterFx, &s);
    assert_eq!(ui.fx_slot, 0);
    assert!(ui.latch);
    ui.latch = false;
    ui.navigate(Page::Mix, &s);
    ui.navigate(Page::TrackFx, &s);
    assert_eq!(ui.fx_slot, 1);
    assert!(ui.latch);
}
#[test]
fn kit_mixer_requires_a_drum_track() {
    let (mut e, mut ui) = state();
    ui.navigate(Page::KitMix, &Snapshot::capture(&e));
    assert_eq!(ui.page, Page::Loop);
    e.tracks[0].kind = Kind::Drum;
    ui.navigate(Page::KitMix, &Snapshot::capture(&e));
    assert_eq!(ui.page, Page::KitMix);
}
#[test]
fn initial_unknown_fader_position_does_not_fake_a_crossing() {
    let mut pickup = Pickup::default();
    pickup.select(0.7);
    assert_eq!(pickup.move_to(0.9, 0.7), None);
    assert_eq!(pickup.move_to(0.8, 0.7), None);
    assert_eq!(pickup.move_to(0.6, 0.7), Some(0.6));
}
#[test]
fn rapid_knob_edits_from_one_snapshot_preserve_prior_changes() {
    let (mut e, mut ui) = state();
    e.tracks[0].kind = Kind::Synth;
    ui.page = Page::Play;
    let s = Snapshot::capture(&e);
    let mut commands = Vec::new();
    ui.knob(0, 0.2, &s, |c| commands.push(c));
    ui.knob(1, 0.8, &s, |c| commands.push(c));
    for c in commands {
        e.command(c).unwrap();
    }
    assert!((e.tracks[0].synth.cutoff - 80. * (18000_f32 / 80.).powf(0.2)).abs() < 0.01);
    assert!((e.tracks[0].synth.attack - 0.401).abs() < 0.001);
}
#[test]
fn kit_mute_solo_and_fader_do_not_overwrite_each_other() {
    let (mut e, mut ui) = state();
    e.tracks[0].kind = Kind::Drum;
    ui.navigate(Page::KitMix, &Snapshot::capture(&e));
    let s = Snapshot::capture(&e);
    let mut commands = Vec::new();
    ui.pad(0, true, 80, false, true, &s, |c| commands.push(c));
    ui.knob(1, 0.4, &s, |c| commands.push(c));
    for c in commands {
        e.command(c).unwrap();
    }
    assert!(e.tracks[0].drums[0].mix.mute);
    assert_eq!(e.tracks[0].drums[0].mix.delay, 0.4);
}

#[test]
fn chop_soft_buttons_cycle_and_apply_modes_without_leaving_editor() {
    let (mut e, mut ui) = state();
    e.tracks[0].audio_frames = 8000;
    e.tracks[0].source_frames = 8000;
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].chops.divide(8000);
    ui.navigate(Page::Chop, &Snapshot::capture(&e));
    for mode in [
        loop_club::chop::Mode::Transient,
        loop_club::chop::Mode::Bpm,
        loop_club::chop::Mode::Manual,
        loop_club::chop::Mode::Equal,
    ] {
        ui.soft(1, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(ui.page, Page::Chop);
        assert_eq!(e.tracks[0].chops.mode, mode);
        assert!(e.tracks[0].chops.pending);
        ui.soft(2, false, &Snapshot::capture(&e), |c| {
            e.command(c).unwrap();
        });
        assert_eq!(ui.page, Page::Chop);
        assert!(!e.tracks[0].chops.pending);
    }
}

#[test]
fn looper_pad_select_is_silent_and_shift_controls_existing_loop_only() {
    let (mut e, mut ui) = state();
    e.tracks[0].length_ticks = 4 * PPQ;
    e.tracks[0].playing = true;
    e.running = true;
    ui.pad(0, true, 90, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert!(e.tracks[0].playing);
    ui.pad(1, true, 90, true, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert!(e.recorder.is_none());
    assert_eq!(e.selected, 1);
    ui.pad(0, true, 90, true, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert!(!e.tracks[0].playing);
}

#[test]
fn changing_sound_requires_fader_pickup_again() {
    let (mut e, mut ui) = state();
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].drums[1].mix.level = 0.3;
    let s = Snapshot::capture(&e);
    ui.navigate(Page::DrumSound, &s);
    ui.fader(0.7, &s, |_| {});
    assert!(ui.pickup.picked);
    ui.pad(1, true, 90, false, false, &s, |_| {});
    assert!(!ui.pickup.picked);
    let mut commands = Vec::new();
    ui.fader(0.69, &s, |c| commands.push(c));
    assert!(commands.is_empty());
    ui.fader(0.3, &s, |c| commands.push(c));
    assert!(matches!(
        commands.last(),
        Some(Command::DrumValue {
            sound: 1,
            index: 0,
            ..
        })
    ));
}
