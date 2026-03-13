//! Uso CPU (percentuale da /proc/stat).
#![allow(dead_code)]

use std::io::BufRead;

fn read_cpu_line() -> Option<(u64, u64)> {
    let f = std::fs::File::open("/proc/stat").ok()?;
    let line = std::io::BufReader::new(f).lines().next().and_then(|r| r.ok())?;
    let parts: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    if parts.len() < 4 {
        return None;
    }
    let total: u64 = parts.iter().sum();
    let idle = parts.get(3).copied().unwrap_or(0);
    Some((total, idle))
}

pub fn text() -> String {
    let (t0, i0) = match read_cpu_line() {
        Some(x) => x,
        None => return String::new(),
    };
    std::thread::sleep(std::time::Duration::from_millis(100));
    let (t1, i1) = match read_cpu_line() {
        Some(x) => x,
        None => return String::new(),
    };
    let total = t1 - t0;
    let idle = i1 - i0;
    if total == 0 {
        return String::new();
    }
    let usage = ((total - idle) * 100) / total;
    format!("{}%", usage)
}
