//! Stato Bluetooth (bluetoothctl o rfkill).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    let out = exec::run("bluetoothctl show 2>/dev/null | grep -E 'Powered|Connected'");
    let powered = out.contains("Powered: yes");
    let connected = out.contains("Connected: yes");
    if !powered {
        return "BT off".to_string();
    }
    if connected {
        return "BT on".to_string();
    }
    "BT".to_string()
}
