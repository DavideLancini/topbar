//! Notifiche (swaync-client -swb, output json con text).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    let out = exec::run("swaync-client -swb 2>/dev/null");
    if out.is_empty() {
        return String::new();
    }
    // JSON: {"text":"..."} o simile
    if let Some(start) = out.find("\"text\":\"") {
        let s = start + 8;
        let end = out[s..].find('"').map(|e| s + e).unwrap_or(out.len());
        return out[s..end].replace("\\\"", "\"");
    }
    String::new()
}
