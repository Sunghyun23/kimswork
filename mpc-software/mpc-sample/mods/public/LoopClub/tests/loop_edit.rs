use loop_club::{
    engine::{Command, Engine, MAX_EVENTS},
    model::*,
    ui::{Action, Snapshot, Ui},
};
fn core() -> Engine {
    let mut e = Engine::new(8000, 100., 16000);
    let t = &mut e.tracks[0];
    t.kind = Kind::Synth;
    t.length_ticks = 4 * PPQ;
    t.events.push(Event {
        deck: None,
        timing: None,
        tick: 0,
        note: 60,
        velocity: 90,
        on: true,
        slice: false,
    });
    t.events.push(Event {
        deck: None,
        timing: None,
        tick: PPQ,
        note: 60,
        velocity: 0,
        on: false,
        slice: false,
    });
    t.synth.cutoff = 0.4;
    t.mix.pan = -0.3;
    t.fx[0].effect = Effect::AutoPan;
    e
}
#[test]
fn copy_is_independent_preserves_sound_and_supports_single_undo_redo() {
    let mut e = core();
    e.transfer_track_stopped(0, 1, false).unwrap();
    assert_eq!(e.tracks[1].kind, Kind::Synth);
    assert_eq!(e.tracks[0].events, e.tracks[1].events);
    assert_eq!(e.tracks[1].synth.cutoff, 0.4);
    assert_eq!(e.tracks[1].mix.pan, -0.3);
    assert_eq!(e.tracks[1].fx[0].effect, Effect::AutoPan);
    assert!(e.tracks[1].events.capacity() >= MAX_EVENTS);
    e.tracks[1].synth.cutoff = 0.8;
    assert_eq!(e.tracks[0].synth.cutoff, 0.4);
    e.command(Command::Undo).unwrap();
    assert!(e.tracks[1].empty());
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[1].synth.cutoff, 0.8);
    assert!(!e.tracks[1].playing);
}
#[test]
fn audio_chop_source_and_multizone_drum_copies_do_not_alias() {
    let mut e = core();
    let t = &mut e.tracks[0];
    t.kind = Kind::Audio;
    t.audio_frames = 800;
    t.source_frames = 800;
    t.source_ticks = 4 * PPQ;
    t.audio_sequence = true;
    t.audio_target = loop_club::audio::RecordTarget::Chop;
    t.audio[..800].fill(Frame::mono(0.2));
    e.transfer_track_stopped(0, 1, false).unwrap();
    assert!(e.tracks[1].audio_sequence);
    e.tracks[1].audio[0] = Frame::ZERO;
    assert_eq!(e.tracks[0].audio[0].l, 0.2);
    e.tracks[2].kind = Kind::Drum;
    e.tracks[2].kit.clips[0] = vec![Frame::mono(0.1); 100];
    let mut x = Box::new(loop_club::xpm::Data::default());
    x.meta.pads[0].count = 2;
    x.extra[0][0] = vec![Frame::mono(0.3); 100];
    e.tracks[2].kit.xpm = Some(x);
    e.transfer_track_stopped(2, 3, false).unwrap();
    e.tracks[3].kit.xpm.as_mut().unwrap().extra[0][0][0] = Frame::ZERO;
    assert_eq!(e.tracks[2].kit.voice_clip(0, 0, 1)[0].l, 0.3);
}
#[test]
fn move_and_undo_keep_chain_sidechain_references() {
    let mut e = core();
    e.chain.config.len = 1;
    e.chain.config.steps[0].track = 0;
    e.sidechain.source = 0;
    e.sidechain.mask = 1;
    e.transfer_track_stopped(0, 4, true).unwrap();
    assert!(e.tracks[0].empty());
    assert_eq!(e.tracks[4].events.len(), 2);
    assert_eq!(e.chain.config.steps[0].track, 4);
    assert_eq!(e.sidechain.source, 4);
    assert_eq!(e.sidechain.mask, 16);
    e.command(Command::Undo).unwrap();
    assert_eq!(e.chain.config.steps[0].track, 0);
    assert_eq!(e.sidechain.source, 0);
    assert_eq!(e.tracks[0].events.len(), 2);
    e.command(Command::Redo).unwrap();
    assert_eq!(e.tracks[4].events.len(), 2);
}
#[test]
fn refuse_running_recording_same_or_occupied_targets_without_touching_content() {
    let mut e = core();
    e.tracks[1].source_frames = 1;
    assert_eq!(
        e.transfer_track_stopped(0, 1, false),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        e.transfer_track_stopped(0, 0, true),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        e.transfer_track_stopped(15, 3, false),
        Err(Error::NoContent)
    );
    e.command(Command::PlayAll).unwrap();
    assert_eq!(e.transfer_track_stopped(0, 3, false), Err(Error::Busy));
    assert_eq!(e.tracks[0].events.len(), 2);
    assert!(e.tracks[3].empty());
}
#[test]
fn later_edits_replace_transfer_history_instead_of_resurrecting_old_tracks() {
    let mut e = core();
    e.transfer_track_stopped(0, 1, false).unwrap();
    e.command(Command::Clear(0)).unwrap();
    e.command(Command::Undo).unwrap();
    assert!(!e.tracks[0].empty());
    assert!(!e.tracks[1].empty());
    e.command(Command::Redo).unwrap();
    assert!(e.tracks[0].empty());
    assert!(!e.tracks[1].empty());
}
#[test]
fn loop_edit_pads_choose_then_transfer_or_confirm_delete_without_performing() {
    let mut e = core();
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    ui.loop_edit = true;
    let mut cmds = Vec::new();
    assert_eq!(
        ui.pad(0, true, 80, false, false, &s, |c| cmds.push(c)),
        Action::None
    );
    assert_eq!(
        ui.pad(1, true, 80, false, false, &s, |c| cmds.push(c)),
        Action::Transfer {
            source: 0,
            target: 1,
            moving: false
        }
    );
    assert!(
        !cmds
            .iter()
            .any(|c| matches!(c, Command::Note { .. } | Command::LoopRecord(_)))
    );
    ui.soft(1, false, &s, |_| {});
    ui.pad(0, true, 80, false, false, &s, |c| cmds.push(c));
    assert!(!cmds.iter().any(|c| matches!(c, Command::Clear(_))));
    ui.pad(0, true, 80, false, false, &s, |c| cmds.push(c));
    assert!(matches!(cmds.last(), Some(Command::Clear(0))));
    e.command(Command::PlayAll).unwrap();
    let s = Snapshot::capture(&e);
    ui.pad(3, true, 80, false, false, &s, |_| {
        panic!("must refuse while playing")
    });
}

#[test]
fn physical_erase_copy_enters_confirmed_edit_and_shift_release_does_not_record() {
    let e = core();
    let s = Snapshot::capture(&e);
    let mut ui = Ui::default();
    assert!(ui.loop_edit_button(9, false, &s));
    assert!(ui.loop_edit);
    assert_eq!(ui.edit_operation, 1);
    let mut commands = Vec::new();
    ui.pad(0, true, 90, false, true, &s, |c| commands.push(c));
    assert!(!commands.iter().any(|c| matches!(c, Command::Clear(_))));
    ui.pad(0, false, 0, false, false, &s, |c| commands.push(c));
    ui.pad(0, true, 90, false, false, &s, |c| commands.push(c));
    assert!(matches!(commands.last(), Some(Command::Clear(0))));
    assert!(ui.loop_edit_button(9, false, &s));
    assert!(!ui.loop_edit);
    assert!(ui.loop_edit_button(9, true, &s));
    assert_eq!(ui.edit_operation, 0);
    assert_eq!(ui.edit_source, None);
    ui.pad(0, true, 90, false, false, &s, |c| commands.push(c));
    assert_eq!(
        ui.pad(1, true, 90, false, false, &s, |c| commands.push(c)),
        Action::Transfer {
            source: 0,
            target: 1,
            moving: false
        }
    );
    assert!(
        !commands
            .iter()
            .any(|c| matches!(c, Command::LoopRecord(_) | Command::Note { .. }))
    );
}

#[test]
fn erase_copy_refuses_transport_and_recording_and_leaves_sound_erase_alone() {
    let mut e = core();
    let mut ui = Ui::default();
    let mut s = Snapshot::capture(&e);
    ui.navigate(Page::Play, &s);
    assert!(!ui.loop_edit_button(9, false, &s));
    assert!(!ui.loop_edit);
    ui.navigate(Page::Loop, &s);
    assert!(!ui.loop_edit_button(74, true, &s));
    e.command(Command::PlayAll).unwrap();
    s = Snapshot::capture(&e);
    assert!(ui.loop_edit_button(9, false, &s));
    assert!(!ui.loop_edit);
    assert_eq!(ui.notice, "STOP BEFORE LOOP EDIT");
    s.words[4] = 0;
    s.words[9] = 1;
    assert!(ui.loop_edit_button(9, true, &s));
    assert!(!ui.loop_edit);
}
