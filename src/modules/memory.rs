//! Uso memoria (percentuale da /proc/meminfo).
#![allow(dead_code)]

use std::io::BufRead;

pub fn text() -> String {
    let mut mem_total: u64 = 0;
    let mut mem_avail: u64 = 0;
    if let Ok(f) = std::fs::File::open("/proc/meminfo") {
        for line in std::io::BufReader::new(f).lines().flatten() {
            if line.starts_with("MemTotal:") {
                mem_total = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            } else if line.starts_with("MemAvailable:") {
                mem_avail = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
        }
    }
    if mem_total == 0 {
        return String::new();
    }
    let used = mem_total.saturating_sub(mem_avail);
    let pct = (used * 100) / mem_total;
    format!("{}%", pct)
}
