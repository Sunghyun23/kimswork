//! Offline numerical QA reference only; never a claimed device recording.
use loop_club::{
    engine::{Command, Engine},
    model::*,
};
use std::io::Write;
fn main() {
    let path = std::env::args().nth(1).expect("output binary path");
    let mut out = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    for scene in 0..3 {
        let mut e = Engine::new(48000, 88., 48000);
        e.running = true;
        for t in 0..16 {
            e.tracks[t].kind = Kind::Synth;
            e.tracks[t].synth.level1 = 0.6;
            e.tracks[t].synth.level2 = 0.5;
            e.tracks[t].mix.pan = (t as f32 / 15. - 0.5) * 0.7;
            if scene == 1 {
                e.tracks[t].synth.lfo_pitch = 60.;
                e.tracks[t].synth.lfo_filter = 2.;
                e.tracks[t].synth.lfo_volume = 0.4;
            }
            for n in [48, 55, 60, 64] {
                e.command(Command::Note {
                    track: t,
                    note: n + (t % 3) as u8,
                    velocity: 40,
                    on: true,
                    source: 2,
                })
                .unwrap();
            }
        }
        for i in 0..96000 {
            if i == 24000 && scene == 1 {
                for t in &mut e.tracks {
                    t.synth.sustain = 0.2;
                    t.synth.filter_sustain = 0.7;
                    t.mix.pan = -t.mix.pan;
                }
            }
            if scene == 2 {
                if i == 12000 {
                    e.performance[0] = Fx {
                        effect: Effect::Reverse,
                        a: 0.4,
                        b: 0.5,
                        mix: 0.6,
                    };
                }
                if i == 24000 {
                    e.performance[0].effect = Effect::Repeat;
                }
                if i == 48000 {
                    e.performance[0].effect = Effect::Off;
                }
            }
            let sample = e.process(Frame::ZERO);
            out.write_all(&sample.l.to_le_bytes()).unwrap();
            out.write_all(&sample.r.to_le_bytes()).unwrap();
        }
    }
    out.flush().unwrap();
}
