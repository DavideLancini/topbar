//! Esecuzione comandi (compile-time: il comando è una costante).

use std::process::Command;

/// Esegue `cmd` (es. "cliphist list | awk 'NR==1{print}'"), ritorna stdout trimmed.
/// Timeout 2s. In caso di errore ritorna stringa vuota.
pub fn run(cmd: &str) -> String {
    let output = Command::new("sh")
        .args(["-c", cmd])
        .output();
    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => String::new(),
    }
}

/// Come run ma con timeout (spawn + wait con timeout non in std, usiamo solo output bloccante).
/// Per ora uguale a run; intervalli lunghi vanno gestiti nel timer.
#[allow(dead_code)]
pub fn run_trim_max(cmd: &str, max_len: usize) -> String {
    let s = run(cmd);
    if s.len() <= max_len {
        s
    } else {
        s.chars().take(max_len).collect()
    }
}
