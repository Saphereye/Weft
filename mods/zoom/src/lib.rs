//! Hold-to-zoom like OptiFine (`Client::view_fov` via `PlayerControl`).

use std::sync::atomic::{AtomicU32, Ordering};

use weft_sdk::*;

declare_mod!(deps: ["base"], init: init);

static ZOOM: Key = Key::named("zoom");
static HELD_FOV: AtomicU32 = AtomicU32::new(1f32.to_bits());
/// While held: FOV scale (lower = closer). 0 = use `scale` default.
static ZOOM_LEVEL: AtomicU32 = AtomicU32::new(0);

static SCALE: Config<f32> = Config::named("scale");
static SCROLL: Config<f32> = Config::named("scroll");
static SMOOTH: Config<f32> = Config::named("smooth");

fn level() -> f32 {
    let bits = ZOOM_LEVEL.load(Ordering::Relaxed);
    if bits == 0 {
        SCALE.get()
    } else {
        f32::from_bits(bits)
    }
}

fn init(r: &mut Registry) {
    r.config(&SCALE).float(0.15, 0.05, 1.0);
    r.config(&SCROLL).float(0.025, 0.005, 0.2);
    r.config(&SMOOTH).float(0.35, 0.05, 1.0);
    r.key(&ZOOM, "Z");
    r.client_frame(|c| {
        let mut target = if c.key_held(&ZOOM) { level() } else { 1.0 };
        if c.key_held(&ZOOM) {
            let wheel = c.scroll_wheel(true);
            if wheel != 0.0 {
                let next = (target - wheel * SCROLL.get()).clamp(0.05, 1.0);
                ZOOM_LEVEL.store(next.to_bits(), Ordering::Relaxed);
                target = next;
            }
        }
        let cur = f32::from_bits(HELD_FOV.load(Ordering::Relaxed));
        let k = SMOOTH.get().clamp(0.05, 1.0);
        let next = cur + (target - cur) * k;
        HELD_FOV.store(next.to_bits(), Ordering::Relaxed);
        c.view_fov(next);
    });
}
