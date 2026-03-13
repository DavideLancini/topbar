//! Frequenza CPU e governor (es. "4.2GHz performance").
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    let freq = exec::run(
        "cat /proc/cpuinfo 2>/dev/null | grep 'cpu MHz' | head -1 | awk '{printf \"%.1f\", $4/1000}'",
    );
    let gov = std::fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .unwrap_or_default()
        .trim()
        .to_string();
    if freq.is_empty() {
        return gov;
    }
    if gov.is_empty() {
        return format!("{}GHz", freq);
    }
    format!("{}GHz {}", freq, gov)
}
