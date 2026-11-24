//! Actual Player replay of logged remote demo positions. No audio is written.
//! Arrival times are host request midpoints: estimates, not measured audio timestamps.
use loop_club::{
    deck::{Player, Settings},
    model::Frame,
};
use serde::Deserialize;
#[derive(Deserialize)]
struct Input {
    at: f64,
    kind: String,
    value: serde_json::Value,
    lateness_ms: f64,
    request_ms: f64,
}
fn run(events: &[Input], actual: bool, quantized: bool) {
    let rate = 44100;
    let cfg = Settings {
        range: 2,
        slip: false,
        ..Settings::default()
    };
    let mut d = Player::default();
    let anchor = if quantized {
        (0.12_f32 * 127.).round() / 127.
    } else {
        0.12
    };
    d.grab(true, Some(anchor), cfg, rate);
    let pcm = vec![Frame::ZERO; rate as usize * 3];
    let mut ix = 0;
    let mut previous = None;
    let mut closed = true;
    let mut speeds = Vec::new();
    let mut open = 0;
    let mut pauses = 0;
    let mut caps = 0;
    let mut maxspeed = 0_f32;
    let arrival = |e: &Input| {
        if actual {
            e.at + e.request_ms / 2000.
        } else {
            e.at - e.lateness_ms / 1000.
        }
    };
    let start = events.iter().map(arrival).fold(f64::INFINITY, f64::min);
    let stop = events.iter().map(arrival).fold(0., f64::max);
    for frame in 0..((stop - start) * f64::from(rate)).ceil() as u64 {
        let t = start + frame as f64 / f64::from(rate);
        while ix < events.len() && arrival(&events[ix]) <= t {
            let e = &events[ix];
            let now = arrival(e);
            if e.kind == "fader" {
                let v = e.value.as_f64().unwrap() as f32;
                let v = if quantized {
                    (v * 127.).round() / 127.
                } else {
                    v
                };
                let us = previous.map(|p: f64| ((now - p) * 1e6).max(0.).round() as u32);
                d.move_to_timed(v, cfg, 100., rate, us);
                previous = Some(now);
            } else if e.kind == "cut" {
                closed = e.value.as_bool().unwrap();
            }
            ix += 1;
        }
        d.sample(&pcm, cfg, rate);
        let speed = d.rate.abs();
        maxspeed = maxspeed.max(speed);
        if !closed {
            open += 1;
            if speed < 0.01 {
                pauses += 1;
            }
            if speed > 3.99 {
                caps += 1;
            }
        }
        let phase = t - start;
        if phase < 4.8 && phase % 0.6 > 0.08 && phase % 0.6 < 0.52 {
            speeds.push(f64::from(speed));
        }
    }
    let mean = speeds.iter().sum::<f64>() / speeds.len() as f64;
    let sd = (speeds.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / speeds.len() as f64).sqrt();
    println!(
        "{}",
        serde_json::json!({"timing":if actual{"estimated_request_midpoint"}else{"nominal_schedule"},"quantized7bit":quantized,"baby_expected_abs_speed":1.2,"baby_steady_mean_abs_speed":mean,"baby_steady_speed_stddev":sd,"open_samples_near_stationary_percent":100.*pauses as f64/open as f64,"open_samples_at_4x_percent":100.*caps as f64/open as f64,"maximum_abs_speed":maxspeed})
    );
}
fn main() {
    let path = std::env::args().nth(1).expect("controls.json path");
    let events: Vec<Input> = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    for (actual, q) in [(false, false), (false, true), (true, true)] {
        run(&events, actual, q);
    }
}
