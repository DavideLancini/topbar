//! Temperatura CPU (sensors, massima).
#![allow(dead_code)]

use crate::exec;

pub fn text() -> String {
    let out = exec::run(
        "sensors 2>/dev/null | awk '/\\+/ && !/crit|high|low/ {for (i=1;i<=NF;i++) if ($i~/^\\+[0-9]+\\.[0-9]+°C$/) print int(substr($i,2,length($i)-3))}' | sort -nr | head -n1",
    );
    if out.is_empty() {
        return String::new();
    }
    format!("{}°C", out)
}
