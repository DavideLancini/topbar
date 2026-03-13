//! Primo elemento dalla lista clipboard (cliphist).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    exec::run_trim_max("cliphist list 2>/dev/null | awk 'NR==1{print}'", 50)
}
