//! Rete: wifi (SSID + segnale) o ethernet (IP).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    // WiFi se presente
    let wifi = exec::run(
        "iwgetid -r 2>/dev/null; iwconfig 2>/dev/null | grep -o 'Quality=[0-9]*' | head -1 | grep -o '[0-9]*'",
    );
    let lines: Vec<&str> = wifi.lines().collect();
    if lines.len() >= 1 && !lines[0].is_empty() {
        let ssid = lines[0];
        let qual = lines.get(1).copied().unwrap_or("").trim();
        if !qual.is_empty() {
            return format!("{}% {}", qual, ssid);
        }
        return ssid.to_string();
    }
    // Ethernet
    let ip = exec::run("ip -4 route get 8.8.8.8 2>/dev/null | grep -oP 'src \\K[0-9.]+\\.[0-9.]+\\.[0-9.]+\\.[0-9]+'");
    if !ip.is_empty() {
        return ip;
    }
    "Disconnected".to_string()
}
