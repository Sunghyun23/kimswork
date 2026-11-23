use loop_club::{
    engine::{Command, Engine},
    model::*,
};
use std::time::Instant;
fn main() {
    let mut results = Vec::new();
    for profile in [
        "16 audio",
        "32 synth voices",
        "32 synth voices + LFO",
        "32 sample drum voices",
        "16 audio + 2 master FX",
    ] {
        let mut e = Engine::new(48000, 88., 48000);
        e.running = true;
        for i in 0..16 {
            let t = &mut e.tracks[i];
            t.length_ticks = 4 * PPQ;
            t.audio_frames = 48000;
            t.audio.fill(Frame::mono(0.01));
            t.playing = true;
            t.mix.delay = 0.1;
            t.mix.room = 0.1;
        }
        if profile.starts_with("32 synth voices") {
            for i in 0..16 {
                e.tracks[i].kind = Kind::Synth;
                if profile.contains("LFO") {
                    e.tracks[i].synth.lfo_pitch = 100.;
                    e.tracks[i].synth.lfo_filter = 3.;
                    e.tracks[i].synth.lfo_volume = 0.7;
                }
                e.tracks[i].audio_frames = 0;
                e.tracks[i].length_ticks = 0;
                for note in [48, 55] {
                    e.command(Command::Note {
                        track: i,
                        note,
                        velocity: 65,
                        on: true,
                        source: 2,
                    })
                    .unwrap();
                }
            }
        }
        if profile == "32 sample drum voices" {
            for track in &mut e.tracks {
                track.kind = Kind::Drum;
                track.audio_frames = 0;
                track.length_ticks = 0;
                for pad in 0..2 {
                    track.kit.clips[pad] = vec![Frame { l: 0.01, r: 0.02 }; 192000];
                    track.drums[pad].decay = 4.;
                }
            }
            for track in 0..16 {
                for note in [0, 1] {
                    e.command(Command::Note {
                        track,
                        note,
                        velocity: 65,
                        on: true,
                        source: 2,
                    })
                    .unwrap();
                }
            }
        }
        if profile.contains("FX") {
            e.performance[0] = Fx {
                effect: Effect::Compressor,
                a: 0.6,
                b: 0.3,
                mix: 0.7,
                ..Fx::default()
            };
            e.performance[1] = Fx {
                effect: Effect::Wobble,
                a: 0.2,
                b: 0.3,
                mix: 0.2,
                ..Fx::default()
            };
        }
        let mut times = Vec::new();
        let mut peak = 0f32;
        for block in 0..1200 {
            let start = Instant::now();
            for _ in 0..128 {
                peak = peak.max(std::hint::black_box(e.process(Frame::ZERO)).peak())
            }
            let t = start.elapsed().as_secs_f64();
            if block > 100 {
                times.push(t)
            }
        }
        times.sort_by(f64::total_cmp);
        let deadline = 128. / 48000.;
        results.push(serde_json::json!({"profile":profile,"native_p99_block_percent":times[times.len()*99/100]/deadline*100.,"native_max_block_percent":times[times.len()-1]/deadline*100.,"peak":peak}));
    }
    println!(
        "{}",
        serde_json::json!({"scope":"Mac native; not hardware CPU certification","results":results})
    );
}
