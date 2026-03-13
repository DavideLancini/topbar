//! Barra nera + testo a (0,0). Niente config: solo buffer e font minimo.

use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use fontdue::{Font, FontSettings};

/// Stesso font e size di Alacritty su questo device (monospace = Noto Sans Mono, size 18).
const FONT_PATH: &str = "/usr/share/fonts/noto/NotoSansMono-Regular.ttf";
const FONT_SIZE: f32 = 18.0;

/// Carica il font (path e size sopra).
pub fn load_font() -> Result<Font, String> {
    let bytes = std::fs::read(Path::new(FONT_PATH))
        .map_err(|e| format!("font {}: {}", FONT_PATH, e))?;
    Font::from_bytes(bytes, FontSettings::default()).map_err(|e| format!("font: {}", e))
}

/// Nessun padding orizzontale: testo a bordo sinistro e destro.
const PAD: i32 = 0;

fn text_width(font: &Font, s: &str) -> i32 {
    s.chars()
        .map(|c| font.metrics(c, FONT_SIZE).advance_width.ceil() as i32)
        .sum()
}

/// 1) Sfondo nero. 2) Tre zone: sinistra (pad), centro (centrato), destra (allineato a destra).
pub fn draw_bar(
    buf: &mut [u8],
    width: u32,
    height: u32,
    font: &Font,
    left_text: &str,
    center_text: &str,
    right_text: &str,
) {
    let stride = (width * 4) as usize;
    if buf.len() < stride * (height as usize) {
        return;
    }

    for y in 0..height {
        for x in 0..width {
            let i = (y as usize * stride) + (x as usize) * 4;
            buf[i..i + 4].copy_from_slice(&[0, 0, 0, 0]);
        }
    }

    let w = width as i32;
    let max_h = left_text.chars().chain(center_text.chars()).chain(right_text.chars())
        .map(|c| font.metrics(c, FONT_SIZE).height as i32)
        .max()
        .unwrap_or(1);
    let target_bottom = (max_h - 1).max(0); // baseline: 1px dal bordo inferiore, nessun margine verticale
    if !left_text.is_empty() {
        draw_line(buf, stride, w, height, font, PAD, target_bottom, left_text);
    }
    if !center_text.is_empty() {
        let cw = text_width(font, center_text);
        let start_x = (w - cw) / 2;
        if start_x >= 0 {
            draw_line(buf, stride, w, height, font, start_x, target_bottom, center_text);
        }
    }
    if !right_text.is_empty() {
        let rw = text_width(font, right_text);
        let start_x = (w - rw - PAD).max(0);
        draw_line(buf, stride, w, height, font, start_x, target_bottom, right_text);
    }
}

/// Una riga di testo: baseline = target_bottom (fondo glifo 1px dal bordo inferiore).
fn draw_line(
    buf: &mut [u8],
    stride: usize,
    width: i32,
    height: u32,
    font: &Font,
    start_x: i32,
    target_bottom: i32,
    s: &str,
) {
    let h = height as i32;
    for c in s.chars() {
    let mut x = start_x;
        let (metrics, bitmap) = font.rasterize(c, FONT_SIZE);
        let glyph_h = metrics.height as i32;
        let origin_y = target_bottom - metrics.ymin - (glyph_h - 1);
        for (i, &alpha) in bitmap.iter().enumerate() {
            if alpha == 0 {
                continue;
            }
            let bx = i % metrics.width;
            let by = i / metrics.width;
            let px_x = x + metrics.xmin + bx as i32;
            let px_y = origin_y + metrics.ymin + by as i32;
            if px_x >= 0 && px_x < width && px_y >= 0 && px_y < h {
                let idx = (px_y as usize * stride) + (px_x as usize) * 4;
                buf[idx] = 255;
                buf[idx + 1] = 255;
                buf[idx + 2] = 255;
                buf[idx + 3] = 0;
            }
        }
        x += metrics.advance_width.ceil() as i32;
    }
}

pub fn write_buf_to_file(f: &mut (impl Write + Seek), buf: &[u8]) -> Result<(), String> {
    f.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    f.write_all(buf).map_err(|e| e.to_string())?;
    f.flush().map_err(|e| e.to_string())?;
    Ok(())
}
