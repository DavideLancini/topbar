//! Batteria: capacità e tempo (da config BATTERY_NAME).
#![allow(dead_code)]

use crate::config;
use std::path::Path;

pub fn text() -> String {
    let base = Path::new("/sys/class/power_supply").join(config::BATTERY_NAME);
    let capacity: u32 = std::fs::read_to_string(base.join("capacity"))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let status = std::fs::read_to_string(base.join("status")).unwrap_or_default();
    let status = status.trim();
    let time_str = std::fs::read_to_string(base.join("time_to_empty_now"))
        .ok()
        .and_then(|s| s.trim().parse::<i32>().ok())
        .map(|m| {
            let h = m / 3600;
            let m = (m % 3600) / 60;
            format!("{}h {}m", h, m)
        })
        .unwrap_or_else(|| {
            std::fs::read_to_string(base.join("time_to_full_avg"))
                .ok()
                .and_then(|s| s.trim().parse::<i32>().ok())
                .map(|m| {
                    let h = m / 3600;
                    let m = (m % 3600) / 60;
                    format!("{}h {}m", h, m)
                })
                .unwrap_or_default()
        });
    let s = match status {
        "Charging" => format!("{}% {}", capacity, time_str),
        "Full" => format!("{}%", capacity),
        _ => format!("{}% {}", capacity, time_str),
    };
    s.trim().to_string()
}
