use loop_club::{
    engine::{Command, Engine},
    model::*,
    project,
};
use std::path::PathBuf;
fn path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "lc-phase-{}-{}.lcproj",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
#[test]
fn saved_stopped_clock_and_independent_loop_origins_preserve_relative_phase() {
    let mut e = Engine::new(8000, 88., 16000);
    e.clock.frame = e.clock.frame_at(64 * PPQ + PPQ / 2);
    for (i, origin) in [4 * PPQ, 16 * PPQ, 36 * PPQ].into_iter().enumerate() {
        let t = &mut e.tracks[i];
        t.kind = Kind::Synth;
        t.origin = origin;
        t.length_ticks = if i == 2 { 4 * PPQ } else { 16 * PPQ };
    }
    let file = path();
    project::save(&e, &file).unwrap();
    let mut restored = project::load(&file, 8000, 16000).unwrap();
    assert_eq!(restored.clock.frame, e.clock.frame);
    e.command(Command::PlayAll).unwrap();
    restored.command(Command::PlayAll).unwrap();
    for _ in 0..512 {
        for i in 0..3 {
            assert_eq!(
                restored.tracks[i].phase(restored.clock.tick()),
                e.tracks[i].phase(e.clock.tick())
            );
        }
        e.process(Frame::ZERO);
        restored.process(Frame::ZERO);
    }
    std::fs::remove_file(file).unwrap();
}
#[test]
fn full_track_copy_preserves_source_origin() {
    let mut e = Engine::new(8000, 88., 16000);
    e.clock.frame = e.clock.frame_at(48 * PPQ);
    e.tracks[0].kind = Kind::Synth;
    e.tracks[0].length_ticks = 16 * PPQ;
    e.tracks[0].origin = 12 * PPQ;
    e.transfer_track_stopped(0, 1, false).unwrap();
    assert_eq!(
        e.tracks[0].phase(e.clock.tick()),
        e.tracks[1].phase(e.clock.tick())
    );
    assert_eq!(e.tracks[0].origin, e.tracks[1].origin);
}
#[test]
fn malformed_transport_position_is_refused_before_engine_creation() {
    let e = Engine::new(8000, 88., 16000);
    let file = path();
    project::save(&e, &file).unwrap();
    let bytes = std::fs::read(&file).unwrap();
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut data: serde_json::Value = serde_json::from_slice(&bytes[8..8 + n]).unwrap();
    data["transport_frame"] = u64::MAX.into();
    let meta = serde_json::to_vec(&data).unwrap();
    let mut changed = b"LC01".to_vec();
    changed.extend((meta.len() as u32).to_le_bytes());
    changed.extend(meta);
    changed.extend(&bytes[8 + n..]);
    std::fs::write(&file, changed).unwrap();
    assert!(project::load(&file, 8000, 16000).is_err());
    std::fs::remove_file(file).unwrap();
}
