//! Topbar — barra Wayland, config a compile-time.

mod config;
mod draw;
mod exec;
mod modules;
mod wayland;

fn main() {
    if let Err(e) = wayland::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
