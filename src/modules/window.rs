//! Hyprland finestra attiva (titolo con rewrite come in waybar).
#![allow(dead_code)]

use crate::exec;

const REWRITES: &[(&str, &str)] = &[
    (" - Visual Studio Code", "VS Code"),
    (" - Vivaldi", "Vivaldi"),
    (" - Cursor", "Cursor"),
    (" - Telegram Desktop", "Telegram"),
];

pub fn text() -> String {
    let out = exec::run("hyprctl activewindow -j 2>/dev/null");
    if out.is_empty() {
        return String::new();
    }
    let title = out
        .find("\"title\":\"")
        .map(|s| {
            let start = s + 9;
            let end = out[start..].find("\"").map(|e| start + e).unwrap_or(out.len());
            out[start..end].replace("\\\"", "\"")
        })
        .unwrap_or_default();
    let mut t = title.as_str();
    for (from, to) in REWRITES {
        if t.ends_with(*from) {
            return to.to_string();
        }
    }
    if t.len() > 20 {
        t = t.get(..20).unwrap_or(t);
    }
    t.to_string()
}
