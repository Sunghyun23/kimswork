//! Independently routed stereo USB stage. Sources/track inserts are rendered once.
use crate::{
    dsp::{Delay, FxState, Room},
    model::*,
};
pub struct UsbStage {
    bus: [FxState; 2],
    perf: [FxState; 2],
    master: FxState,
    delay: Delay,
    room: Room,
}
impl UsbStage {
    pub fn new(rate: u32) -> Self {
        Self {
            bus: std::array::from_fn(|_| FxState::new(rate)),
            perf: std::array::from_fn(|_| FxState::new(rate)),
            master: FxState::new(rate),
            delay: Delay::new(rate),
            room: Room::new(rate),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &mut self,
        mut sum: Frame,
        buses: [Frame; 2],
        delay: Frame,
        room: Frame,
        rate: f32,
        bpm: f32,
        bus_fx: [Fx; 2],
        performance: [Fx; 2],
        master_fx: Fx,
        gain: f32,
        texture_state: &mut crate::motion::TextureState,
        texture: crate::motion::Texture,
        duck: f32,
    ) -> Frame {
        for (i, bus) in buses.into_iter().enumerate() {
            sum = sum.combine(self.bus[i].process(bus, bus_fx[i], rate, bpm));
        }
        sum = sum
            .combine(self.delay.process(delay, rate, bpm))
            .combine(self.room.process(room));
        for (i, (stage, mut fx)) in self.perf.iter_mut().zip(performance).enumerate() {
            if texture.master_bypass[i] {
                fx.effect = Effect::Off;
            }
            sum = stage.process(sum, fx, rate, bpm);
        }
        sum = self.master.process(sum, master_fx, rate, bpm);
        sum = texture_state
            .process(sum, texture, rate, bpm)
            .gain(gain * duck);
        if !sum.finite() {
            return Frame::ZERO;
        }
        Frame {
            l: sum.l.clamp(-0.98, 0.98),
            r: sum.r.clamp(-0.98, 0.98),
        }
    }
}
/// Fixed-path, bounded read-only diagnostics. Never changes USB gadget configuration.
pub fn capabilities() -> serde_json::Value {
    use std::{fs, path::Path};
    fn read(p: impl AsRef<Path>) -> Option<String> {
        let p = p.as_ref();
        if fs::metadata(p).ok()?.len() > 65536 {
            return None;
        }
        let mut f = std::fs::File::open(p).ok()?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut std::io::Read::take(&mut f, 65536), &mut bytes).ok()?;
        Some(String::from_utf8_lossy(&bytes).into_owned())
    }
    let mut attrs = serde_json::Map::new();
    let root = Path::new("/sys/kernel/config/usb_gadget");
    if let Ok(gadgets) = fs::read_dir(root) {
        for g in gadgets.take(8).flatten() {
            let functions = g.path().join("functions");
            if let Ok(fs) = fs::read_dir(functions) {
                for f in fs.take(32).flatten() {
                    let name = f.file_name().to_string_lossy().into_owned();
                    if name.contains("uac") {
                        for key in [
                            "p_chmask", "c_chmask", "p_srate", "c_srate", "p_ssize", "c_ssize",
                        ] {
                            if let Some(value) = read(f.path().join(key)) {
                                attrs.insert(format!("{}/{key}", name), value.trim().into());
                            }
                        }
                    }
                }
            }
        }
    }
    let mut udc = serde_json::Map::new();
    if let Ok(entries) = fs::read_dir("/sys/class/udc") {
        for e in entries.take(8).flatten() {
            for key in ["current_speed", "maximum_speed"] {
                if let Some(value) = read(e.path().join(key)) {
                    udc.insert(
                        format!("{}/{key}", e.file_name().to_string_lossy()),
                        value.trim().into(),
                    );
                }
            }
        }
    }
    let mut params = serde_json::Map::new();
    for module in ["g_audio", "usb_f_uac2", "u_audio"] {
        for key in ["p_chmask", "c_chmask", "p_srate", "c_srate"] {
            if let Some(value) = read(format!("/sys/module/{module}/parameters/{key}")) {
                params.insert(format!("{module}/{key}"), value.trim().into());
            }
        }
    }
    serde_json::json!({"scope":"bounded read-only OS USB/audio capabilities; no USB configuration changes","kernel":read("/proc/sys/kernel/osrelease"),"alsa_cards":read("/proc/asound/cards"),"alsa_pcm":read("/proc/asound/pcm"),"gadget_audio":attrs,"module_parameters":params,"usb_controller":udc})
}
