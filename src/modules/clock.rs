//! Orario (formato in config: %H:%M %d/%m/%y).
#![allow(dead_code)]

use crate::config;
use crate::exec;

pub fn text() -> String {
    exec::run("date '+%H:%M %d/%m/%y' 2>/dev/null")
}

#[allow(dead_code)]
pub fn interval_secs() -> u64 {
    config::CLOCK_INTERVAL_SECS
}
