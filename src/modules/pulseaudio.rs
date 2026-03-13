//! Volume e sink (pactl).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    let vol = exec::run(
        "pactl get-sink-volume @DEFAULT_SINK@ 2>/dev/null | head -1 | sed -n 's/.* \\([0-9]*\\)%.*/\\1/p'",
    );
    let mute = exec::run("pactl get-sink-mute @DEFAULT_SINK@ 2>/dev/null");
    let muted = mute.contains("yes");
    if vol.is_empty() {
        return String::new();
    }
    if muted {
        return format!("{}% (muted)", vol);
    }
    format!("{}%", vol)
}
