//! Local optional-bank audit; no device access or public asset generation.
use loop_club::{
    bank::Bank,
    engine::{Command, Engine},
    model::*,
};
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let path = std::path::Path::new(args.get(1).expect("bank path required"));
    let rate = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(48000u32);
    let all = args.get(3).is_some_and(|s| s == "all");
    let bank = Bank::open(path)?;
    let mut e = Engine::new(rate, 88., rate as usize * 4);
    e.tracks[0].kind = Kind::Drum;
    let indices: Vec<_> = if all {
        (0..bank.manifest.kits.len()).collect()
    } else {
        let mut v = vec![0, bank.manifest.kits.len() - 1];
        v.sort();
        v.dedup();
        v
    };
    let mut checked = 0;
    let mut max_frames = 0;
    let mut max_bytes = 0;
    let mut peak = 0f32;
    for index in indices {
        let mut job = bank.prepare(index, 0, rate)?;
        max_bytes = max_bytes.max(job.kit.clips.iter().map(|c| c.len() * 8).sum::<usize>());
        max_frames = max_frames.max(job.kit.clips.iter().map(Vec::len).max().unwrap_or(0));
        if job.kit.clips.iter().any(|c| c.iter().any(|x| !x.finite())) {
            return Err(std::io::Error::other("Nonfinite PCM"));
        }
        job.commit(&mut e);
        if job.error.is_some() {
            return Err(std::io::Error::other("Commit failed"));
        }
        for pad in 0..16 {
            if !e.tracks[0].kit.clips[pad].is_empty() {
                e.command(Command::Note {
                    track: 0,
                    note: pad as u8,
                    velocity: 90,
                    on: true,
                    source: 2,
                })
                .unwrap();
            }
        }
        for _ in 0..rate / 10 {
            let sample = e.process(Frame::ZERO);
            if !sample.finite() {
                return Err(std::io::Error::other("Nonfinite engine output"));
            }
            peak = peak.max(sample.peak());
        }
        checked += 1;
    }
    println!(
        "{}",
        serde_json::json!({"scope":"native bank parsing/decoding/engine playback, not hardware audio","bank":bank.manifest.name,"rate":rate,"records":bank.manifest.records.len(),"kits_total":bank.manifest.kits.len(),"kits_checked":checked,"max_decoded_frames":max_frames,"max_kit_bytes":max_bytes,"output_peak":peak,"status":"PASS"})
    );
    Ok(())
}
