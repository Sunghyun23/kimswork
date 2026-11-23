use loop_club::{
    engine::{Command, Engine},
    model::*,
    project,
    ui::{Snapshot, Ui},
};
fn engine(target: OutputTarget) -> Engine {
    let mut e = Engine::new(8000, 240., 8000);
    e.tracks[0].kind = Kind::Drum;
    e.tracks[0].kit.clips[0] = vec![Frame::mono(0.3); 1000];
    e.command(Command::Output(0, target)).unwrap();
    e.command(Command::Note {
        track: 0,
        note: 0,
        velocity: 127,
        on: true,
        source: 2,
    })
    .unwrap();
    e
}
#[test]
fn main_usb_both_are_isolated_and_source_advances_once() {
    for target in [OutputTarget::Main, OutputTarget::Usb, OutputTarget::Both] {
        let mut e = engine(target);
        let mut main = 0f32;
        let mut usb = 0f32;
        for _ in 0..500 {
            let x = e.process(Frame::ZERO);
            main = main.max(x.peak());
            usb = usb.max(e.usb_output.peak());
        }
        match target {
            OutputTarget::Main => {
                assert!(main > 0.01);
                assert_eq!(usb, 0.);
            }
            OutputTarget::Usb => {
                assert_eq!(main, 0.);
                assert!(usb > 0.01);
            }
            OutputTarget::Both => {
                assert!(main > 0.01);
                assert_eq!(main, usb);
            }
        }
    }
}
#[test]
fn usb_only_does_not_feed_main_room_or_fx_bus() {
    let mut e = engine(OutputTarget::Usb);
    e.tracks[0].route = Route::A;
    e.tracks[0].mix.delay = 0.8;
    e.tracks[0].mix.room = 0.8;
    e.bus_fx[0].effect = Effect::Drive;
    e.performance[0].effect = Effect::Filter;
    let mut usb = 0f32;
    for _ in 0..12000 {
        assert_eq!(e.process(Frame::ZERO), Frame::ZERO);
        usb = usb.max(e.usb_output.peak());
    }
    assert!(usb > 0.01);
}
#[test]
fn output_selection_roundtrips_and_legacy_defaults_both() {
    let mut e = Engine::new(8000, 240., 8000);
    e.command(Command::Output(3, OutputTarget::Usb)).unwrap();
    let p = std::env::temp_dir().join(format!("lc-usb-{}.lcproj", std::process::id()));
    project::save(&e, &p).unwrap();
    let loaded = project::load(&p, 8000, 8000).unwrap();
    assert_eq!(loaded.tracks[3].output, OutputTarget::Usb);
    assert!(loaded.separate_usb);
    assert_eq!(loaded.tracks[3].main_gain, 0.);
    let bytes = std::fs::read(&p).unwrap();
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut meta: serde_json::Value = serde_json::from_slice(&bytes[8..8 + n]).unwrap();
    for t in meta["tracks"].as_array_mut().unwrap() {
        t.as_object_mut().unwrap().remove("output");
    }
    let json = serde_json::to_vec(&meta).unwrap();
    let mut out = b"LC01".to_vec();
    out.extend((json.len() as u32).to_le_bytes());
    out.extend(json);
    out.extend(&bytes[8 + n..]);
    std::fs::write(&p, out).unwrap();
    let legacy = project::load(&p, 8000, 8000).unwrap();
    assert!(legacy.tracks.iter().all(|t| t.output == OutputTarget::Both));
    assert!(!legacy.separate_usb);
    std::fs::remove_file(p).unwrap();
}
#[test]
fn route_editor_changes_only_selected_track_output() {
    let mut e = Engine::new(8000, 240., 8000);
    let mut ui = Ui::default();
    ui.navigate(Page::Output, &Snapshot::capture(&e));
    ui.knob(0, 0.5, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(e.tracks[0].output, OutputTarget::Usb);
    assert_eq!(e.tracks[1].output, OutputTarget::Both);
    ui.soft(2, false, &Snapshot::capture(&e), |c| e.command(c).unwrap());
    assert_eq!(ui.selected, 1);
    ui.pad(0, true, 80, false, false, &Snapshot::capture(&e), |c| {
        e.command(c).unwrap()
    });
    assert_eq!(e.tracks[1].output, OutputTarget::Main);
}

#[test]
fn hardware_volume_scales_main_and_usb_without_rewriting_saved_master() {
    for target in [OutputTarget::Main, OutputTarget::Usb, OutputTarget::Both] {
        let mut e = engine(target);
        e.tracks[0].kit.clips[0] = vec![Frame::mono(0.3); 8000];
        e.command(Command::HardwareVolume(0.)).unwrap();
        for _ in 0..6000 {
            e.process(Frame::ZERO);
        }
        let quiet = e.process(Frame::ZERO);
        assert!(quiet.peak() < 1e-6 && e.usb_output.peak() < 1e-6);
        assert_eq!(e.master, 0.8);
        e.command(Command::HardwareVolume(0.75)).unwrap();
        e.command(Command::Note {
            track: 0,
            note: 0,
            velocity: 127,
            on: true,
            source: 2,
        })
        .unwrap();
        let (mut main, mut usb) = (0_f32, 0_f32);
        for _ in 0..3000 {
            main = main.max(e.process(Frame::ZERO).peak());
            usb = usb.max(e.usb_output.peak());
        }
        match target {
            OutputTarget::Main => assert!(main > 0.01 && usb == 0.),
            OutputTarget::Usb => assert!(main == 0. && usb > 0.01),
            OutputTarget::Both => assert!(main > 0.01 && usb > 0.01),
        }
    }
}
#[test]
fn project_replacement_preserves_live_hardware_volume_and_selected_track() {
    let p = std::env::temp_dir().join(format!("lc-live-volume-{}.lcproj", std::process::id()));
    let mut saved = Engine::new(8000, 88., 8000);
    saved.master = 0.6;
    project::save(&saved, &p).unwrap();
    let mut live = Engine::new(8000, 88., 8000);
    live.hardware_volume = 0.2;
    live.hardware_gain = 0.21;
    live.selected = 7;
    let restored = project::load_for_replacement(&p, 8000, 8000, &live).unwrap();
    assert_eq!(restored.hardware_volume, 0.2);
    assert_eq!(restored.hardware_gain, 0.21);
    assert_eq!(restored.master, 0.6);
    assert_eq!(restored.selected, 7);
    std::fs::remove_file(p).unwrap();
}
