//! Read-only native measurement of the actual deck implementation; writes no audio.
use loop_club::{
    deck::{Player, Settings},
    model::Frame,
};
fn measure(hz: u32, quantize: bool) {
    let rate = 44100u32;
    let bpm = 100.;
    let cfg = Settings {
        range: 1,
        slip: false,
        ..Settings::default()
    };
    let pcm: Vec<_> = (0..rate * 3)
        .map(|i| Frame::mono((i as f32 * 0.017).sin() * 0.2))
        .collect();
    let mut deck = Player::default();
    deck.start(44100.);
    deck.grab(true, Some(0.), cfg, rate);
    let length = rate * 3 / 10;
    let interval = rate / hz;
    let expected = 0.45 * 0.6 / 0.3;
    let mut speeds = Vec::new();
    let mut edge_values = Vec::new();
    let mut trace = Vec::new();
    for i in 0..length {
        if i % interval == 0 {
            let mut position = 0.45 * (i + interval).min(length) as f32 / length as f32;
            if quantize {
                position = (position * 127.).round() / 127.;
            }
            deck.move_to(position, cfg, bpm, rate);
        }
        deck.sample(&pcm, cfg, rate);
        speeds.push(deck.rate as f64);
        if hz == 10 && !quantize && i < interval && i % 44 == 0 {
            trace.push((i as f64 / 44.1, deck.rate));
        }
        if i % interval == interval - 1 {
            edge_values.push(deck.rate);
        }
    }
    let mean = speeds.iter().sum::<f64>() / speeds.len() as f64;
    let sd = (speeds.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / speeds.len() as f64).sqrt();
    let near_zero = speeds.iter().filter(|x| x.abs() < 0.01).count() as f64 / speeds.len() as f64;
    let cap = speeds.iter().filter(|x| x.abs() > 3.99).count() as f64 / speeds.len() as f64;
    println!(
        "{}",
        serde_json::json!({"updates_hz":hz,"quantized7bit":quantize,"ideal_speed":expected,"mean_speed":mean,"speed_stddev":sd,"near_stationary_fraction":near_zero,"at_4x_cap_fraction":cap,"last_speed_each_interval":edge_values,"first_interval_trace_ms_speed":trace})
    );
}
fn main() {
    for hz in [10, 20, 50, 100, 200] {
        measure(hz, false);
        measure(hz, true);
    }
}
