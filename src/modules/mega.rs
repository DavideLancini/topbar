//! Stato MEGA sync (output di mega-sync).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    exec::run_trim_max("mega-sync 2>/dev/null | awk 'NR==2{print $5}'", 30)
}
