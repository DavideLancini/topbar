//! Luminosità (da config path).
#![allow(dead_code)]

use crate::config;

pub fn text() -> String {
    let brightness = std::fs::read_to_string(format!("{}/brightness", config::BACKLIGHT_PATH))
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);
    let max_brightness = std::fs::read_to_string(format!("{}/max_brightness", config::BACKLIGHT_PATH))
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(100);
    let pct = if max_brightness > 0 {
        (brightness * 100) / max_brightness
    } else {
        0
    };
    format!("{}%", pct)
}
