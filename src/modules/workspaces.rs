//! Hyprland workspaces (format: "1 2 3", active dal monitor).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    let workspaces = exec::run("hyprctl workspaces -j 2>/dev/null");
    let active = exec::run("hyprctl activeworkspace -j 2>/dev/null");
    if workspaces.is_empty() || active.is_empty() {
        // Sotto Hyprland ma hyprctl fallisce (es. PATH): mostra placeholder
        if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            return "—".to_string();
        }
        return String::new();
    }
    // hyprctl -j può avere spazi/newline: "id": 2
    let active_id: i32 = active
        .find("\"id\":")
        .and_then(|pos| {
            let rest = &active[pos + 5..];
            let num_str: String = rest
                .chars()
                .skip_while(|c| c.is_ascii_whitespace())
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect();
            num_str.parse().ok()
        })
        .unwrap_or(1);
    let mut ids: Vec<i32> = Vec::new();
    let mut it = workspaces.split("\"id\":");
    it.next(); // skip first
    for part in it {
        let num_str: String = part
            .chars()
            .skip_while(|c| c.is_ascii_whitespace())
            .take_while(|c| c.is_numeric() || *c == '-')
            .collect();
        if let Ok(num) = num_str.parse::<i32>() {
            ids.push(num);
        }
    }
    ids.sort();
    ids.dedup();
    let fmt: Vec<String> = ids
        .iter()
        .map(|&id| {
            if id == active_id {
                format!("[{}]", id)
            } else {
                id.to_string()
            }
        })
        .collect();
    fmt.join(" ")
}
