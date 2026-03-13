//! Le 3 zone della barra. Per ora solo "testo" in tutte; i moduli veri si aggiungono uno alla volta.

#[allow(dead_code)]
fn join(non_empty: &[String]) -> String {
    let v: Vec<&str> = non_empty.iter().filter(|s| !s.is_empty()).map(String::as_str).collect();
    v.join(" | ")
}

pub fn left() -> String {
    "testo".to_string()
}

pub fn center() -> String {
    "testo".to_string()
}

pub fn right() -> String {
    "testo".to_string()
}
