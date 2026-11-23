use loop_club::{
    engine::{Command, Engine},
    model::*,
    project,
    xpm::Program,
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "lc-xpm-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(root.join("Programs")).unwrap();
        std::fs::create_dir_all(root.join("Samples")).unwrap();
        std::fs::write(root.join("Expansion.xml"), "<Expansion/>").unwrap();
        let mut w = b"RIFF".to_vec();
        w.extend(1636u32.to_le_bytes());
        w.extend(b"WAVEfmt ");
        w.extend(16u32.to_le_bytes());
        w.extend(1u16.to_le_bytes());
        w.extend(1u16.to_le_bytes());
        w.extend(8000u32.to_le_bytes());
        w.extend(16000u32.to_le_bytes());
        w.extend(2u16.to_le_bytes());
        w.extend(16u16.to_le_bytes());
        w.extend(b"data");
        w.extend(1600u32.to_le_bytes());
        for i in 0..800 {
            w.extend(((i as i16) * 20).to_le_bytes());
        }
        std::fs::write(root.join("Samples/Tone.WAV"), w).unwrap();
        Self(root)
    }
    fn program(&self, body: &str) -> std::io::Result<Program> {
        let path = self.0.join("Programs/Test.xpm");
        std::fs::write(
            &path,
            format!(
                "<MPCVObject><Program type=\"Drum\"><ProgramName>Generated Kit</ProgramName><Instruments>{body}</Instruments></Program></MPCVObject>"
            ),
        )?;
        Program::open(&path, &self.0)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn layer(n: usize, fields: &str) -> String {
    format!("<Layer number=\"{n}\"><SampleFile>..\\samples\\tone.wav</SampleFile>{fields}</Layer>")
}
fn pad(n: usize, fields: &str, layers: &str) -> String {
    format!("<Instrument number=\"{n}\">{fields}<Layers>{layers}</Layers></Instrument>")
}
#[test]
fn bank_selection_preserves_empty_positions_and_decodes_only_selected_bank() {
    let f = Fixture::new();
    let a = pad(
        1,
        "",
        "<Layer number=\"1\"><SampleName>Missing</SampleName></Layer>",
    );
    let b = pad(18, "", &layer(1, ""));
    let h = pad(128, "", &layer(1, ""));
    let p = f.program(&format!("{a}{b}{h}")).unwrap();
    assert_eq!(p.banks, vec![0, 1, 7]);
    assert!(p.prepare(0, 0, 8000).is_err());
    let j = p.prepare(1, 0, 8000).unwrap();
    assert_eq!(j.kit.clips[1].len(), 800);
    assert!(j.kit.clips[0].is_empty());
    assert!(!j.drums[0].layers[0].enabled);
    assert!(j.drums[1].layers[0].enabled);
    assert!(p.prepare(2, 0, 8000).is_err());
    let j = p.prepare(7, 0, 8000).unwrap();
    assert_eq!(j.kit.clips[15].len(), 800);
}
#[test]
fn four_velocity_zones_and_native_rules_survive_portable_project() {
    let f = Fixture::new();
    let layers = (1..=4)
        .map(|i| {
            layer(
                i,
                &format!(
                    "<VelStart>{}</VelStart><VelEnd>{}</VelEnd>",
                    (i - 1) * 32,
                    i * 32 - 1
                ),
            )
        })
        .collect::<String>();
    let p=f.program(&pad(17,"<ZonePlay>1</ZonePlay><MuteGroup>3</MuteGroup><MuteTarget1>18</MuteTarget1><TriggerMode>1</TriggerMode><Mono>True</Mono>",&layers)).unwrap();
    let mut j = p.prepare(1, 0, 8000).unwrap();
    let meta = j.kit.xpm.as_ref().unwrap().meta;
    assert!(meta.valid());
    assert_eq!(meta.pads[0].count, 4);
    assert_eq!(meta.pads[0].ranges[3], [96, 127]);
    assert_eq!(meta.pads[0].mute_targets[0], 2);
    assert!(meta.pads[0].gate && meta.pads[0].mono);
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Drum;
    j.commit(&mut e);
    assert!(j.error.is_none());
    let path = f.0.join("portable");
    project::save(&e, &path).unwrap();
    std::fs::remove_file(f.0.join("Samples/Tone.WAV")).unwrap();
    let restored = project::load(&path, 8000, 16000).unwrap();
    assert_eq!(restored.tracks[0].kit.voice_clip(0, 0, 3).len(), 800);
    assert_eq!(
        restored.tracks[0].kit.xpm.as_ref().unwrap().meta.pads[0].count,
        4
    );
}
#[test]
fn trim_reverse_gain_and_pitch_are_applied_before_callback() {
    let f = Fixture::new();
    let p=f.program(&pad(1,"",&layer(1,"<SliceStart>100</SliceStart><SliceEnd>500</SliceEnd><Direction>1</Direction><Volume>0.5</Volume>"))).unwrap();
    let j = p.prepare(0, 0, 8000).unwrap();
    let pcm = &j.kit.clips[0];
    assert_eq!(pcm.len(), 400);
    assert!(pcm[0].l > pcm[399].l * 4.);
    assert!(pcm[0].peak() < 0.16);
    let p = f
        .program(&pad(1, "", &layer(1, "<TuneCoarse>12</TuneCoarse>")))
        .unwrap();
    assert_eq!(p.prepare(0, 0, 8000).unwrap().kit.clips[0].len(), 400);
}
#[test]
fn malformed_metadata_is_rejected_before_import() {
    let f = Fixture::new();
    for body in [
        pad(129, "", &layer(1, "")),
        pad(
            1,
            "",
            &layer(1, "<VelStart>100</VelStart><VelEnd>20</VelEnd>"),
        ),
        pad(1, "<ZonePlay>3</ZonePlay>", &layer(1, "")),
        pad(1, "", &format!("{}{}", layer(1, ""), layer(1, ""))),
        pad(1, "", &layer(5, "")),
        pad(1, "<Volume>NaN</Volume>", &layer(1, "")),
    ] {
        assert!(f.program(&body).is_err(), "{body}");
    }
    let path = f.0.join("Programs/Test.xpm");
    std::fs::write(
        &path,
        "<!DOCTYPE x [<!ENTITY foo 'bar'>]><MPCVObject><Program type=\"Drum\"/></MPCVObject>",
    )
    .unwrap();
    assert!(Program::open(&path, &f.0).is_err());
}
#[test]
fn file_aliases_do_not_allow_ambiguous_sample_search() {
    let f = Fixture::new();
    std::fs::create_dir(f.0.join("Other")).unwrap();
    std::fs::copy(f.0.join("Samples/Tone.WAV"), f.0.join("Other/Tone.wav")).unwrap();
    let p = f
        .program(&pad(
            1,
            "",
            "<Layer number=\"1\"><SampleName>Tone</SampleName></Layer>",
        ))
        .unwrap();
    assert!(p.prepare(0, 0, 8000).is_err());
    let p = f.program(&pad(1, "", &layer(1, ""))).unwrap();
    assert!(p.prepare(0, 0, 8000).is_ok());
}
#[test]
fn replacing_full_kit_keeps_other_track_and_rejects_recording_target() {
    let f = Fixture::new();
    let p = f.program(&pad(1, "", &layer(1, ""))).unwrap();
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Synth;
    let mut j = p.prepare(0, 0, 8000).unwrap();
    j.commit(&mut e);
    assert_eq!(j.error, Some(Error::WrongKind));
    assert!(e.tracks[0].kit.clips[0].is_empty());
    e.tracks[0].kind = Kind::Drum;
    e.tracks[1].length_ticks = 4 * PPQ;
    e.tracks[1].playing = true;
    let mut j = p.prepare(0, 0, 8000).unwrap();
    j.commit(&mut e);
    assert!(j.error.is_none());
    assert!(e.tracks[1].playing);
    e.command(Command::Note {
        track: 0,
        note: 0,
        velocity: 96,
        on: true,
        source: 1,
    })
    .unwrap();
}

#[test]
fn velocity_cycle_random_gate_and_choke_have_distinct_runtime_behavior() {
    fn note(e: &mut Engine, pad: u8, velocity: u8, on: bool) {
        e.command(Command::Note {
            track: 0,
            note: pad,
            velocity,
            on,
            source: 2,
        })
        .unwrap();
    }
    let f = Fixture::new();
    let layers = (1..=4).map(|n| layer(n, "")).collect::<String>();
    for mode in 0..=2 {
        let a = pad(
            1,
            &format!(
                "<ZonePlay>{mode}</ZonePlay><MuteGroup>1</MuteGroup><Mono>True</Mono><TriggerMode>1</TriggerMode>"
            ),
            &layers,
        );
        let b = pad(2, "<MuteGroup>1</MuteGroup>", &layer(1, ""));
        let p = f.program(&(a + &b)).unwrap();
        let mut j = p.prepare(0, 0, 8000).unwrap();
        let mut e = Engine::new(8000, 100., 16000);
        e.tracks[0].kind = Kind::Drum;
        j.commit(&mut e);
        for _ in 0..6 {
            note(&mut e, 0, 80, true);
            assert_eq!(e.active_voices(), if mode == 1 { 4 } else { 1 });
        }
        note(&mut e, 0, 0, false);
        for _ in 0..100 {
            let _ = e.process(Frame::ZERO);
        }
        assert_eq!(e.active_voices(), 0);
        note(&mut e, 0, 80, true);
        note(&mut e, 1, 80, true);
        assert_eq!(e.active_voices(), 1);
        assert_eq!(
            e.tracks[0].xpm_step[0],
            if mode == 0 {
                7
            } else if mode == 1 {
                0
            } else {
                e.tracks[0].xpm_step[0]
            }
        );
    }
    let layers = layer(1, "<VelStart>0</VelStart><VelEnd>63</VelEnd>")
        + &layer(2, "<VelStart>64</VelStart><VelEnd>127</VelEnd>");
    let p = f.program(&pad(1, "<Mono>True</Mono>", &layers)).unwrap();
    let mut j = p.prepare(0, 0, 8000).unwrap();
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Drum;
    j.commit(&mut e);
    note(&mut e, 0, 16, true);
    assert_eq!(e.active_voices(), 1);
    note(&mut e, 0, 112, true);
    assert_eq!(e.active_voices(), 1);
}

#[test]
fn changing_source_kind_keeps_dormant_xpm_zones_project_portable() {
    let f = Fixture::new();
    let layers = layer(1, "") + &layer(2, "");
    let p = f.program(&pad(1, "", &layers)).unwrap();
    let mut job = p.prepare(0, 0, 8000).unwrap();
    let mut e = Engine::new(8000, 100., 16000);
    e.tracks[0].kind = Kind::Drum;
    job.commit(&mut e);
    for kind in [Kind::Synth, Kind::Audio, Kind::Drum] {
        e.command(Command::SetKind(0, kind)).unwrap();
        let path = f.0.join("switch-project");
        project::save(&e, &path).unwrap();
        let loaded = project::load(&path, 8000, 16000).unwrap();
        assert_eq!(loaded.tracks[0].kind, kind);
        assert_eq!(loaded.tracks[0].kit.voice_clip(0, 0, 1).len(), 800);
    }
}
