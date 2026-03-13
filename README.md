# topbar

Barra di stato Wayland: **sfondo nero**, **3 zone di testo** (sinistra, centro, destra). Config a tempo di compilazione.

## Design

- **Nessun elemento cliccabile**, nessun overlay.
- Ogni zona è **un’unica stringa di testo**. Elementi multipli (es. workspaces + clipboard a sinistra) sono **concatenati** e mostrati come testo (es. `1 2 [3] | clip...`).
- La barra ridisegna periodicamente le 3 stringhe; non gestisce eventi di input.

## Requisiti

- Compositor Wayland con **layer-shell** (Hyprland, Sway, River, …).
- Rust (stable).

## Build ed esecuzione

```bash
cargo build --release
./target/release/topbar
```

Oppure `./run.sh` (build + avvio).

## Configurazione

Tutto in **`src/config.rs`**: altezza, colori, font, path (battery, backlight, timer, …). Modifica e ricompila.

Cosa va in ogni zona è definito in **`src/modules/layout.rs`** (left / center / right).

## Licenza

MIT
