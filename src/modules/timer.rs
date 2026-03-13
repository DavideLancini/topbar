//! Timer (script status, path in config).
#![allow(dead_code)]

use crate::config;
use crate::exec;

pub fn text() -> String {
    let out = exec::run(&format!("{} status 2>/dev/null", config::TIMER_SCRIPT));
    if out.is_empty() {
        return String::new();
    }
    // JSON: {"text":"...","tooltip":"..."} o solo text
    if let Some(start) = out.find("\"text\":\"") {
        let s = start + 8;
        let end = out[s..].find('"').map(|e| s + e).unwrap_or(out.len());
        return out[s..end].replace("\\\"", "\"");
    }
    out
}
